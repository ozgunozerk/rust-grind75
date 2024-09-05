use std::collections::HashMap;

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        self.parent[y] = x
    }

    fn find(&mut self, x: usize) -> usize {
        // path compression
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
}

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut hm = HashMap::new();
        let mut uf = UnionFind::new(accounts.len());
        for (i, account) in accounts.iter().enumerate() {
            for email in account.iter().skip(1) {
                if let Some(&j) = hm.get(email) {
                    uf.union(i, j);
                } else {
                    hm.insert(email.to_owned(), i);
                }
            }
        }

        let mut components = HashMap::new();
        for (email, i) in hm {
            components
                .entry(uf.find(i))
                .or_insert(Vec::new())
                .push(email);
        }

        let mut answer = vec![];
        for (i, mut emails) in components {
            let mut merged = vec![accounts[i][0].clone()];
            emails.sort_unstable();
            merged.extend(emails);
            answer.push(merged);
        }
        answer
    }
}
