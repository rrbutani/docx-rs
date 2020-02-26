#[cfg(not(test))]
use std::sync::atomic::AtomicUsize;
#[cfg(not(test))]
static HISTORY_ID: AtomicUsize = AtomicUsize::new(1);

#[cfg(not(test))]
pub trait HistoryId {
    fn generate(&self) -> String {
        use std::sync::atomic::Ordering;

        let id = HISTORY_ID.load(Ordering::Relaxed);
        HISTORY_ID.store(id + 1, Ordering::Relaxed);
        format!("{}", id)
    }

    fn clear_history_id(&self) {
        use std::sync::atomic::Ordering;
        HISTORY_ID.store(1, Ordering::Relaxed);
    }
}

#[cfg(test)]
pub trait HistoryId {
    fn generate(&self) -> &str {
        "123"
    }

    fn clear_history_id(&self) {}
}
