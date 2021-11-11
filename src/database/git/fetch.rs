use super::Repo;

impl Repo<'_> {
    pub fn fetch(&mut self) {
        let mut origin_remote = self.repo.find_remote("origin").unwrap();
        origin_remote
            .fetch(&["master"], Some(&mut self.fetch_options), None)
            .unwrap();
        let oid = self
            .repo
            .refname_to_id("refs/remotes/origin/master")
            .unwrap();
        let object = self.repo.find_object(oid, None).unwrap();
        self.repo
            .reset(&object, git2::ResetType::Hard, None)
            .unwrap();
    }
}
