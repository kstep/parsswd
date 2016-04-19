#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PwEnt<'a> {
    pub name: &'a str,
    pub passwd: &'a str,
    pub uid: u32,
    pub gid: u32,
    pub gecos: &'a str,
    pub home_dir: &'a str,
    pub shell: &'a str,
}

impl<'a> PwEnt<'a> {
    pub fn from_str(s: &'a str) -> Option<PwEnt<'a>> {
        let mut entries = s.splitn(7, ':');
        Some(PwEnt {
            name: match entries.next() { None => return None, Some(s) => s },
            passwd: match entries.next() { None => return None, Some(s) => s },
            uid: match entries.next().and_then(|s| s.parse().ok()) { None => return None, Some(s) => s },
            gid: match entries.next().and_then(|s| s.parse().ok()) { None => return None, Some(s) => s },
            gecos: match entries.next() { None => return None, Some(s) => s },
            home_dir: match entries.next() { None => return None, Some(s) => s },
            shell: match entries.next() { None => return None, Some(s) => s },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrpEnt<'a> {
    pub name: &'a str,
    pub passwd: &'a str,
    pub gid: u32,
    pub users: Vec<&'a str>,
}

impl<'a> GrpEnt<'a> {
    pub fn from_str(s: &'a str) -> Option<GrpEnt<'a>> {
        let mut entries = s.splitn(4, ':');
        Some(GrpEnt {
            name: match entries.next() { None => return None, Some(s) => s },
            passwd: match entries.next() { None => return None, Some(s) => s },
            gid: match entries.next().and_then(|s| s.parse().ok()) { None => return None, Some(s) => s },
            users: match entries.next() { None => return None, Some(s) => s.split(',').collect() },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{PwEnt, GrpEnt};

    #[test]
    fn pw_ent_works() {
        let root_line = "root:x:0:0:root:/root:/bin/bash";
        let pwent = PwEnt::from_str(root_line).unwrap();
        assert_eq!(pwent, PwEnt {
            name: "root",
            passwd: "x",
            uid: 0,
            gid: 0,
            gecos: "root",
            home_dir: "/root",
            shell: "/bin/bash",
        });
    }

    #[test]
    fn grp_ent_works() {
        let daemon_line = "daemon:x:2:root,bin,daemon";
        let grent = GrpEnt::from_str(daemon_line).unwrap();
        assert_eq!(grent, GrpEnt {
            name: "daemon",
            passwd: "x",
            gid: 2,
            users: vec!["root", "bin", "daemon"],
        });
    }
}
