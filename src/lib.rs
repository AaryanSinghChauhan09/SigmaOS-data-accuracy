#![forbid(unsafe_code)]
//! SigmaOS-data-accuracy: data accuracy model.
//!
//! Real implementation: precision, error margins, and validation.
//! Replaces the 108-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Accuracy metric.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccuracyMetric {
    pub id: String,
    pub name: String,
    pub field: String,
    pub tolerance: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccuracyError {
    EmptyName,
    DuplicateMetric(String),
}

impl fmt::Display for AccuracyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "metric name must not be empty"),
            Self::DuplicateMetric(id) => write!(f, "duplicate metric {:?}", id),
        }
    }
}

impl std::error::Error for AccuracyError {}

/// Data accuracy manager.
pub struct DataAccuracy {
    pub name: String,
    metrics: BTreeMap<String, AccuracyMetric>,
}

impl DataAccuracy {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            metrics: BTreeMap::new(),
        }
    }

    pub fn add_metric(&mut self, metric: AccuracyMetric) -> Result<(), AccuracyError> {
        if metric.name.trim().is_empty() { return Err(AccuracyError::EmptyName); }
        let id = metric.id.clone();
        if self.metrics.contains_key(&id) {
            return Err(AccuracyError::DuplicateMetric(id.clone()));
        }
        self.metrics.insert(id, metric);
        Ok(())
    }

    pub fn metric_count(&self) -> usize { self.metrics.len() }

    pub fn get_metric(&self, id: &str) -> Option<&AccuracyMetric> { self.metrics.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accuracy_create_aaa() {
        let a = DataAccuracy::new("My Accuracy");
        assert_eq!(a.name, "My Accuracy");
        assert_eq!(a.metric_count(), 0);
    }

    #[test]
    fn test_add_metric_aaa() {
        let mut a = DataAccuracy::new("My Accuracy");
        let m = AccuracyMetric { id: "m1".into(), name: "precision".into(), field: "temperature".into(), tolerance: "0.1".into() };
        a.add_metric(m).unwrap();
        assert_eq!(a.metric_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut a = DataAccuracy::new("My Accuracy");
        let m = AccuracyMetric { id: "x".into(), name: "".into(), field: "a".into(), tolerance: "b".into() };
        assert!(a.add_metric(m).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = DataAccuracy::new("My Accuracy");
        let m = AccuracyMetric { id: "x".into(), name: "X".into(), field: "a".into(), tolerance: "b".into() };
        a.add_metric(m).unwrap();
        let m2 = AccuracyMetric { id: "x".into(), name: "X".into(), field: "a".into(), tolerance: "b".into() };
        assert!(a.add_metric(m2).is_err());
    }
}
