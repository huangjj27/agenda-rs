use std::io;

struct User {
    name: String,
    pass: String,
    email: Option<String>,
    name: Option<String>,
}

impl User {
    fn register(n: String, p: String, em: Option<String>, pho: Option<String>) -> bool;
    fn login(n: String, p: String) -> bool;
    fn delete(n: String, p: String) -> bool;

    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, n: String) -> Result<(), io::Error> {
        if n.len() == 0 {
            return Err(io::ErrorKind::InValidInput, "name shouldn't be empty!");
        }

        self.name = n;
    }

    fn pass(&self) -> &str {
        &self.pass
    }

    fn set_pass(&mut self, p: String) -> Result<(), io::Error> {
        if n.len() <= 6 {
            return Err(io::ErrorKind::InValidInput, "password shouldn't be empty, or less than 6 characters!");
        }

        self.pass = p;
    }

    fn email(&self) -> Option<&str> {
        &self.email
    }

    fn set_email(&self, em: String) {
        if em.len() == 0 {
            self.email = None;
            return;
        }

        self.email = parse_email(em);
    }

    fn phone(&self) -> Option<&str> {
        &self.phone
    }

    fn set_phone(&self, pho: String) {
        if pho.len() == 0 {
            self.pho = None;
            return;
        }

        self.phone = parse_phone(pho);
    }
}
