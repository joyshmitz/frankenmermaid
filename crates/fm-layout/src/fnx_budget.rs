//! Budget enforcement for FNX analysis operations.
//!
//! This module provides time and work budgets for fnx analysis calls with
//! deterministic cancellation semantics. Key invariants:
//!
//! - **Determinism**: Budget enforcement produces consistent fallback behavior
//! - **Safety**: Timeouts never cause panics or partial results without explanation
//! - **Observability**: All budget events are traced with structured diagnostics
//!
//! # Budget Model
//!
//! Each analysis operation has:
//! - A time budget (wall-clock milliseconds, default 100ms)
//! - A work budget (iterations or operations, default 10000)
//! - A fallback strategy when budget is exceeded
//!
//! # Usage
//!
//! ```ignore
//! let budget = AnalysisBudget::default();
//! let result = budget.execute(|| expensive_fnx_operation());
//! match result {
//!     BudgetResult::Completed(value) => { /* success */ }
//!     BudgetResult::TimedOut(reason) => { /* handle gracefully */ }
//! }
//! ```

use std::time::Instant;

// ============================================================================
// Budget Configuration
// ============================================================================

/// Configuration for analysis time and work budgets.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnalysisBudget {
    /// Maximum wall-clock time for this operation in milliseconds.
    pub time_budget_ms: u64,
    /// Maximum number of work units (algorithm-specific iterations).
    pub work_budget: u64,
    /// Whether to emit detailed traces for budget consumption.
    pub trace_enabled: bool,
}

impl Default for AnalysisBudget {
    fn default() -> Self {
        Self {
            time_budget_ms: 100,  // 100ms default
            work_budget: 10000,   // 10k iterations default
            trace_enabled: false,
        }
    }
}

impl AnalysisBudget {
    /// Create a new budget with custom time limit.
    #[must_use]
    pub const fn with_time_budget_ms(mut self, ms: u64) -> Self {
        self.time_budget_ms = ms;
        self
    }

    /// Create a new budget with custom work limit.
    #[must_use]
    pub const fn with_work_budget(mut self, work: u64) -> Self {
        self.work_budget = work;
        self
    }

    /// Enable or disable detailed tracing.
    #[must_use]
    pub const fn with_trace(mut self, enabled: bool) -> Self {
        self.trace_enabled = enabled;
        self
    }

    /// Strict budget: 50ms time, 5000 work units.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            time_budget_ms: 50,
            work_budget: 5000,
            trace_enabled: false,
        }
    }

    /// Relaxed budget: 500ms time, 50000 work units.
    #[must_use]
    pub const fn relaxed() -> Self {
        Self {
            time_budget_ms: 500,
            work_budget: 50000,
            trace_enabled: false,
        }
    }

    /// Unlimited budget for testing/debugging only.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            time_budget_ms: u64::MAX,
            work_budget: u64::MAX,
            trace_enabled: false,
        }
    }
}

// ============================================================================
// Budget Enforcement
// ============================================================================

/// Reason for budget-triggered cancellation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BudgetExceededReason {
    /// Time budget was exceeded.
    TimeExceeded {
        /// Budget in milliseconds.
        budget_ms: u64,
        /// Actual elapsed time in milliseconds.
        elapsed_ms: u64,
    },
    /// Work budget was exceeded.
    WorkExceeded {
        /// Budget in work units.
        budget: u64,
        /// Actual work units consumed.
        consumed: u64,
    },
    /// Both time and work budgets were exceeded.
    BothExceeded {
        /// Time info.
        budget_ms: u64,
        elapsed_ms: u64,
        /// Work info.
        work_budget: u64,
        work_consumed: u64,
    },
}

impl BudgetExceededReason {
    /// Format as a diagnostic message.
    #[must_use]
    pub fn as_diagnostic(&self) -> String {
        match self {
            Self::TimeExceeded { budget_ms, elapsed_ms } => {
                format!("FNX analysis exceeded time budget: {elapsed_ms}ms > {budget_ms}ms limit")
            }
            Self::WorkExceeded { budget, consumed } => {
                format!("FNX analysis exceeded work budget: {consumed} > {budget} iterations")
            }
            Self::BothExceeded { budget_ms, elapsed_ms, work_budget, work_consumed } => {
                format!(
                    "FNX analysis exceeded budgets: time {elapsed_ms}ms > {budget_ms}ms, work {work_consumed} > {work_budget}"
                )
            }
        }
    }

    /// Get a short reason code for structured logging.
    #[must_use]
    pub fn reason_code(&self) -> &'static str {
        match self {
            Self::TimeExceeded { .. } => "time_exceeded",
            Self::WorkExceeded { .. } => "work_exceeded",
            Self::BothExceeded { .. } => "time_and_work_exceeded",
        }
    }
}

/// Result of a budget-constrained operation.
#[derive(Debug, Clone)]
pub enum BudgetResult<T> {
    /// Operation completed within budget.
    Completed {
        /// The result value.
        value: T,
        /// Time consumed in milliseconds.
        elapsed_ms: u64,
        /// Work units consumed (if tracked).
        work_consumed: Option<u64>,
    },
    /// Operation was cancelled due to budget exceeded.
    Cancelled {
        /// Why the operation was cancelled.
        reason: BudgetExceededReason,
        /// Partial result if available.
        partial: Option<T>,
    },
}

impl<T> BudgetResult<T> {
    /// Returns true if the operation completed within budget.
    #[must_use]
    pub fn is_completed(&self) -> bool {
        matches!(self, Self::Completed { .. })
    }

    /// Returns true if the operation was cancelled.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        matches!(self, Self::Cancelled { .. })
    }

    /// Get the result value, whether complete or partial.
    /// Returns None if cancelled with no partial result.
    #[must_use]
    pub fn into_value(self) -> Option<T> {
        match self {
            Self::Completed { value, .. } => Some(value),
            Self::Cancelled { partial, .. } => partial,
        }
    }

    /// Get the cancellation reason if cancelled.
    #[must_use]
    pub fn cancellation_reason(&self) -> Option<&BudgetExceededReason> {
        match self {
            Self::Cancelled { reason, .. } => Some(reason),
            Self::Completed { .. } => None,
        }
    }

    /// Get elapsed time in milliseconds.
    #[must_use]
    pub fn elapsed_ms(&self) -> u64 {
        match self {
            Self::Completed { elapsed_ms, .. } => *elapsed_ms,
            Self::Cancelled { reason, .. } => match reason {
                BudgetExceededReason::TimeExceeded { elapsed_ms, .. }
                | BudgetExceededReason::BothExceeded { elapsed_ms, .. } => *elapsed_ms,
                BudgetExceededReason::WorkExceeded { .. } => 0,
            },
        }
    }
}

// ============================================================================
// Budget Executor
// ============================================================================

/// Executes operations with budget enforcement.
#[derive(Debug, Clone)]
pub struct BudgetExecutor {
    budget: AnalysisBudget,
    start_time: Option<Instant>,
    work_consumed: u64,
}

impl BudgetExecutor {
    /// Create a new executor with the given budget.
    #[must_use]
    pub fn new(budget: AnalysisBudget) -> Self {
        Self {
            budget,
            start_time: None,
            work_consumed: 0,
        }
    }

    /// Start the budget clock.
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.work_consumed = 0;
    }

    /// Record work units consumed.
    pub fn record_work(&mut self, units: u64) {
        self.work_consumed = self.work_consumed.saturating_add(units);
    }

    /// Check if time budget is exceeded.
    #[must_use]
    pub fn is_time_exceeded(&self) -> bool {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed().as_millis() as u64;
            elapsed > self.budget.time_budget_ms
        } else {
            false
        }
    }

    /// Check if work budget is exceeded.
    #[must_use]
    pub fn is_work_exceeded(&self) -> bool {
        self.work_consumed > self.budget.work_budget
    }

    /// Check if any budget is exceeded.
    #[must_use]
    pub fn is_exceeded(&self) -> bool {
        self.is_time_exceeded() || self.is_work_exceeded()
    }

    /// Get elapsed time in milliseconds.
    #[must_use]
    pub fn elapsed_ms(&self) -> u64 {
        self.start_time
            .map(|s| s.elapsed().as_millis() as u64)
            .unwrap_or(0)
    }

    /// Get current work consumption.
    #[must_use]
    pub fn work_consumed(&self) -> u64 {
        self.work_consumed
    }

    /// Build the exceeded reason if budget was exceeded.
    #[must_use]
    pub fn exceeded_reason(&self) -> Option<BudgetExceededReason> {
        let time_exceeded = self.is_time_exceeded();
        let work_exceeded = self.is_work_exceeded();

        match (time_exceeded, work_exceeded) {
            (true, true) => Some(BudgetExceededReason::BothExceeded {
                budget_ms: self.budget.time_budget_ms,
                elapsed_ms: self.elapsed_ms(),
                work_budget: self.budget.work_budget,
                work_consumed: self.work_consumed,
            }),
            (true, false) => Some(BudgetExceededReason::TimeExceeded {
                budget_ms: self.budget.time_budget_ms,
                elapsed_ms: self.elapsed_ms(),
            }),
            (false, true) => Some(BudgetExceededReason::WorkExceeded {
                budget: self.budget.work_budget,
                consumed: self.work_consumed,
            }),
            (false, false) => None,
        }
    }

    /// Execute a closure with budget enforcement.
    ///
    /// Note: This only checks the budget after the operation completes.
    /// For true pre-emption, operations need to check `is_exceeded()` internally.
    pub fn execute<T, F: FnOnce() -> T>(&mut self, f: F) -> BudgetResult<T> {
        self.start();
        let value = f();
        let elapsed_ms = self.elapsed_ms();

        if let Some(reason) = self.exceeded_reason() {
            if self.budget.trace_enabled {
                tracing::warn!(
                    reason = reason.reason_code(),
                    elapsed_ms,
                    work_consumed = self.work_consumed,
                    "FNX analysis budget exceeded"
                );
            }
            BudgetResult::Cancelled {
                reason,
                partial: Some(value),
            }
        } else {
            if self.budget.trace_enabled {
                tracing::debug!(
                    elapsed_ms,
                    work_consumed = self.work_consumed,
                    "FNX analysis completed within budget"
                );
            }
            BudgetResult::Completed {
                value,
                elapsed_ms,
                work_consumed: Some(self.work_consumed),
            }
        }
    }

    /// Execute with a progress callback that can check budget.
    ///
    /// The callback receives a reference to the executor to check `is_exceeded()`.
    pub fn execute_with_progress<T, F>(&mut self, f: F) -> BudgetResult<T>
    where
        F: FnOnce(&mut Self) -> T,
    {
        self.start();
        let value = f(self);
        let elapsed_ms = self.elapsed_ms();

        if let Some(reason) = self.exceeded_reason() {
            if self.budget.trace_enabled {
                tracing::warn!(
                    reason = reason.reason_code(),
                    elapsed_ms,
                    work_consumed = self.work_consumed,
                    "FNX analysis budget exceeded"
                );
            }
            BudgetResult::Cancelled {
                reason,
                partial: Some(value),
            }
        } else {
            if self.budget.trace_enabled {
                tracing::debug!(
                    elapsed_ms,
                    work_consumed = self.work_consumed,
                    "FNX analysis completed within budget"
                );
            }
            BudgetResult::Completed {
                value,
                elapsed_ms,
                work_consumed: Some(self.work_consumed),
            }
        }
    }
}

// ============================================================================
// Global Budget Context
// ============================================================================

// Thread-local budget context for nested operations.
thread_local! {
    static BUDGET_CONTEXT: std::cell::RefCell<Option<BudgetContext>> = const { std::cell::RefCell::new(None) };
}

/// Global budget context for tracking nested analysis operations.
#[derive(Debug, Clone)]
pub struct BudgetContext {
    /// Stack of active budget executors.
    executors: Vec<BudgetExecutor>,
    /// Total time consumed across all operations.
    total_elapsed_ms: u64,
    /// Total work consumed across all operations.
    total_work: u64,
    /// Whether any operation was cancelled.
    any_cancelled: bool,
    /// Cancellation reasons collected.
    cancellation_reasons: Vec<BudgetExceededReason>,
}

impl Default for BudgetContext {
    fn default() -> Self {
        Self::new()
    }
}

impl BudgetContext {
    /// Create a new budget context.
    #[must_use]
    pub fn new() -> Self {
        Self {
            executors: Vec::new(),
            total_elapsed_ms: 0,
            total_work: 0,
            any_cancelled: false,
            cancellation_reasons: Vec::new(),
        }
    }

    /// Push a new budget executor onto the stack.
    pub fn push(&mut self, budget: AnalysisBudget) {
        self.executors.push(BudgetExecutor::new(budget));
    }

    /// Pop the current budget executor.
    pub fn pop(&mut self) -> Option<BudgetExecutor> {
        if let Some(executor) = self.executors.pop() {
            self.total_elapsed_ms += executor.elapsed_ms();
            self.total_work += executor.work_consumed();
            if let Some(reason) = executor.exceeded_reason() {
                self.any_cancelled = true;
                self.cancellation_reasons.push(reason);
            }
            Some(executor)
        } else {
            None
        }
    }

    /// Get the current (innermost) budget executor.
    #[must_use]
    pub fn current(&self) -> Option<&BudgetExecutor> {
        self.executors.last()
    }

    /// Get mutable reference to current executor.
    pub fn current_mut(&mut self) -> Option<&mut BudgetExecutor> {
        self.executors.last_mut()
    }

    /// Check if any active budget is exceeded.
    #[must_use]
    pub fn any_exceeded(&self) -> bool {
        self.executors.iter().any(BudgetExecutor::is_exceeded)
    }

    /// Get total elapsed time in milliseconds.
    #[must_use]
    pub fn total_elapsed_ms(&self) -> u64 {
        self.total_elapsed_ms
            + self
                .executors
                .iter()
                .map(BudgetExecutor::elapsed_ms)
                .sum::<u64>()
    }

    /// Get all cancellation reasons.
    #[must_use]
    pub fn cancellation_reasons(&self) -> &[BudgetExceededReason] {
        &self.cancellation_reasons
    }

    /// Whether any operation was cancelled.
    #[must_use]
    pub fn any_cancelled(&self) -> bool {
        self.any_cancelled || self.executors.iter().any(BudgetExecutor::is_exceeded)
    }
}

/// Enter a budget scope in the thread-local context.
pub fn enter_budget_scope(budget: AnalysisBudget) {
    BUDGET_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        if ctx.is_none() {
            *ctx = Some(BudgetContext::new());
        }
        if let Some(ref mut c) = *ctx {
            c.push(budget);
            if let Some(executor) = c.current_mut() {
                executor.start();
            }
        }
    });
}

/// Exit the current budget scope.
pub fn exit_budget_scope() -> Option<BudgetExecutor> {
    BUDGET_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        ctx.as_mut().and_then(BudgetContext::pop)
    })
}

/// Check if the current budget is exceeded (from within an operation).
pub fn is_budget_exceeded() -> bool {
    BUDGET_CONTEXT.with(|ctx| {
        ctx.borrow()
            .as_ref()
            .map(BudgetContext::any_exceeded)
            .unwrap_or(false)
    })
}

/// Record work units in the current budget scope.
pub fn record_budget_work(units: u64) {
    BUDGET_CONTEXT.with(|ctx| {
        if let Some(executor) = ctx.borrow_mut().as_mut().and_then(|c| c.current_mut()) {
            executor.record_work(units);
        }
    });
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn default_budget_values() {
        let budget = AnalysisBudget::default();
        assert_eq!(budget.time_budget_ms, 100);
        assert_eq!(budget.work_budget, 10000);
        assert!(!budget.trace_enabled);
    }

    #[test]
    fn budget_presets() {
        let strict = AnalysisBudget::strict();
        assert_eq!(strict.time_budget_ms, 50);
        assert_eq!(strict.work_budget, 5000);

        let relaxed = AnalysisBudget::relaxed();
        assert_eq!(relaxed.time_budget_ms, 500);
        assert_eq!(relaxed.work_budget, 50000);
    }

    #[test]
    fn executor_time_tracking() {
        let mut executor = BudgetExecutor::new(AnalysisBudget::default());
        executor.start();
        std::thread::sleep(Duration::from_millis(10));
        assert!(executor.elapsed_ms() >= 10);
    }

    #[test]
    fn executor_work_tracking() {
        let mut executor = BudgetExecutor::new(
            AnalysisBudget::default().with_work_budget(100),
        );
        executor.start();
        executor.record_work(50);
        assert_eq!(executor.work_consumed(), 50);
        assert!(!executor.is_work_exceeded());

        executor.record_work(60);
        assert_eq!(executor.work_consumed(), 110);
        assert!(executor.is_work_exceeded());
    }

    #[test]
    fn executor_execute_within_budget() {
        let mut executor = BudgetExecutor::new(AnalysisBudget::relaxed());
        let result = executor.execute(|| 42);

        assert!(result.is_completed());
        assert_eq!(result.into_value(), Some(42));
    }

    #[test]
    fn executor_execute_with_time_exceeded() {
        let mut executor = BudgetExecutor::new(
            AnalysisBudget::default().with_time_budget_ms(1),
        );
        let result = executor.execute(|| {
            std::thread::sleep(Duration::from_millis(10));
            42
        });

        assert!(result.is_cancelled());
        // Should still have partial result
        assert_eq!(result.into_value(), Some(42));
    }

    #[test]
    fn executor_execute_with_work_exceeded() {
        let mut executor = BudgetExecutor::new(
            AnalysisBudget::default().with_work_budget(50),
        );
        let result = executor.execute_with_progress(|exec| {
            exec.record_work(100);
            "done"
        });

        assert!(result.is_cancelled());
        let reason = result.cancellation_reason().unwrap();
        assert_eq!(reason.reason_code(), "work_exceeded");
    }

    #[test]
    fn exceeded_reason_diagnostics() {
        let reason = BudgetExceededReason::TimeExceeded {
            budget_ms: 100,
            elapsed_ms: 150,
        };
        assert!(reason.as_diagnostic().contains("150ms > 100ms"));

        let reason = BudgetExceededReason::WorkExceeded {
            budget: 1000,
            consumed: 1500,
        };
        assert!(reason.as_diagnostic().contains("1500 > 1000"));
    }

    #[test]
    fn budget_context_nested_scopes() {
        enter_budget_scope(AnalysisBudget::default());
        assert!(!is_budget_exceeded());

        enter_budget_scope(AnalysisBudget::strict());
        assert!(!is_budget_exceeded());

        record_budget_work(100);

        let inner = exit_budget_scope();
        assert!(inner.is_some());
        assert_eq!(inner.unwrap().work_consumed(), 100);

        let outer = exit_budget_scope();
        assert!(outer.is_some());
    }

    #[test]
    fn budget_result_methods() {
        let completed: BudgetResult<i32> = BudgetResult::Completed {
            value: 42,
            elapsed_ms: 10,
            work_consumed: Some(100),
        };
        assert!(completed.is_completed());
        assert!(!completed.is_cancelled());
        assert!(completed.cancellation_reason().is_none());
        assert_eq!(completed.elapsed_ms(), 10);

        let cancelled: BudgetResult<i32> = BudgetResult::Cancelled {
            reason: BudgetExceededReason::TimeExceeded {
                budget_ms: 50,
                elapsed_ms: 100,
            },
            partial: Some(21),
        };
        assert!(!cancelled.is_completed());
        assert!(cancelled.is_cancelled());
        assert!(cancelled.cancellation_reason().is_some());
        assert_eq!(cancelled.elapsed_ms(), 100);
    }

    #[test]
    fn thread_local_isolation() {
        // Verify thread-local context doesn't leak
        enter_budget_scope(AnalysisBudget::default());
        record_budget_work(500);

        let handle = std::thread::spawn(|| {
            // Different thread should have independent context
            assert!(!is_budget_exceeded());
            enter_budget_scope(AnalysisBudget::strict());
            record_budget_work(100);
            exit_budget_scope()
        });

        let other_executor = handle.join().unwrap();
        assert!(other_executor.is_some());
        assert_eq!(other_executor.unwrap().work_consumed(), 100);

        // Original thread's context should be unchanged
        let executor = exit_budget_scope();
        assert_eq!(executor.unwrap().work_consumed(), 500);
    }
}
