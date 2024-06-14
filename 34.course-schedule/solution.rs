use std::collections::VecDeque;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut in_degree = vec![0; num_courses as usize];
        let mut unlocks = vec![vec![]; num_courses as usize];

        for req in prerequisites.iter() {
            unlocks[req[1] as usize].push(req[0]);
            in_degree[req[0] as usize] += 1;
        }

        let mut queue = VecDeque::with_capacity(num_courses as usize);
        for (course, &degree) in in_degree.iter().enumerate() {
            if degree == 0 {
                queue.push_back(course);
            }
        }

        let mut finished = 0;
        while let Some(course) = queue.pop_front() {
            finished += 1;

            for unlocked_course in unlocks[course].iter() {
                let degree = in_degree.get_mut(*unlocked_course as usize).unwrap();
                *degree -= 1;

                if *degree == 0 {
                    queue.push_back(*unlocked_course as usize);
                }
            }
        }

        finished == num_courses
    }
}
