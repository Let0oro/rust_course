/** The PartialOrd Trait indicates that a type can be ordered/sorted

    Is similar to PartialEq (is a subtrait of PartialEq) but return an Option Ordering enum

*/
use std::cmp::Ordering::{self, Less, Greater, Equal};


struct Job {
    salary: u32,
    commute_time: u32
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.salary == other.salary
    }
}

impl Eq for Job {}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        /// Cause u32 implements trait PartialOrd and its method partial_cmp (partial compare), we can do:
        // match self.salary.partial_cmp(&other.salary) {
        //     Some(Less) => Some(Less),
        //     Some(Greater) => Some(Greater),
        //     Some(Equal) => Some(Equal),
        //     None => None,
        // }

        /// or even
        self.salary.partial_cmp(&other.salary)

        // if self.salary < other.salary {Some(Less)}
        // else if self.salary == other.salary {Some(Greater)}
        // else if self.salary > other.salary {Some(Greater)}
        // else { None }
    }
}

pub fn main () {

}