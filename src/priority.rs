#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A dynamic list which labels elements by priority
/// ```
/// use opencl3_select::PriorityList;
/// let mut prio_list = PriorityList::new();
///
/// prio_list.push(1);
/// prio_list.push(3);
/// prio_list.push(77);
///
/// let remaining = prio_list
///     .view_remaining()
///     .into_iter()
///     .collect::<Vec<_>>();
///
/// assert_eq!(remaining[0], &1);
/// assert_eq!(remaining[1], &3);
/// assert_eq!(remaining[2], &77);
/// ```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PriorityList<T> {
    selected: Vec<T>,
    remaining: Vec<T>,
}

impl<T> PriorityList<T> {
    /// Construct a new empty [PriorityList]
    /// ```
    /// use opencl3_select::PriorityList;
    /// let priorty_list: PriorityList<u8> = PriorityList::new();
    /// ```
    pub fn new() -> Self {
        Self {
            selected: Vec::new(),
            remaining: Vec::new(),
        }
    }

    /// Gets the highest priority member
    pub fn priority_first(&self) -> Option<&T> {
        self.selected.first()
    }

    /// Gets the nth-highest priority member
    pub fn priority_nth(&self, n: usize) -> Option<&T> {
        self.selected.iter().nth(n)
    }

    /// Adds another member to the priority list
    pub fn push(&mut self, element: T) {
        self.remaining.push(element);
    }

    /// Adds another element and sets it as the first priority
    pub fn push_set_first(&mut self, element: T) {
        self.selected.insert(0, element)
    }

    /// View the current priority list
    pub fn view_priority_list(&self) -> impl IntoIterator<Item=&T> {
        self.selected.iter()
    }

    /// View the remaining items which are not selected for priority
    pub fn view_remaining(&self) -> impl IntoIterator<Item=&T> {
        self.remaining.iter()
    }

    /// Selects a currently not selected item in the priority
    /// list with currently lowest priority
    pub fn select(&mut self, n: usize) {
        if self.remaining.len() < n {
            let selected = self.remaining.remove(n);
            self.selected.push(selected);
        }
    }

    /// Selects an element and puts it at the nth position of the list
    pub fn select_set_nth(&mut self, n: usize, priority_level: usize) {
        if self.remaining.len() < n {
            let selected = self.remaining.remove(n);
            self.selected.insert(priority_level, selected);
        }
    }

    /// See [select_set_nth](PriorityList::select_set_nth)
    pub fn select_set_first(&mut self, n: usize) {
        self.select_set_nth(n, 0);
    }
}

impl<T> From<Vec<T>> for PriorityList<T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            selected: value,
            remaining: Vec::new(),
        }
    }
}
