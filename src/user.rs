trait User {
    fn register(n: String, p: String, em: String, pho: String) -> bool;
    fn login(n: String, p: String) -> bool;
    fn delete(n: String, p: String) -> bool;

    fn name(&self) -> &str;
    fn name_mut(&mut self) -> &mut String;

    fn pass(&self) -> &str;
    fn pass_mut(&mut self) -> &mut String;

    fn email(&self) -> Option<&str>;
    fn email(&self) -> Option<&mut String>;

    fn phone(&self) -> Option<&str>;
    fn phone_mut(&self) -> Option<&mut String>;
}

// struct User {
//
// }
