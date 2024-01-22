use std::collections::VecDeque;
struct Solution;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph = vec![Vec::new(); num_courses as usize];
        let mut in_degree = vec![0; num_courses as usize];

        prerequisites.iter().for_each(|prereq| {
            let (course, prereq_course) = (prereq[0], prereq[1]);
            graph[prereq_course as usize].push(course);
            in_degree[course as usize] += 1;
        });

        let mut queue = VecDeque::with_capacity(num_courses as usize);
        queue.extend(
            in_degree
                .iter()
                .enumerate()
                .filter(|(_course, &degree)| degree == 0)
                .map(|(course, _)| course as i32),
        );

        let mut count = 0;

        while let Some(course) = queue.pop_front() {
            count += 1;

            if let Some(neighbors) = graph.get(course as usize) {
                for &next_course in neighbors {
                    in_degree[next_course as usize] -= 1;
                    if in_degree[next_course as usize] == 0 {
                        queue.push_back(next_course);
                    }
                }
            }
        }

        count == num_courses
    }
}
