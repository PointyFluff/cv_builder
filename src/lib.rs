#![warn(rust_2018_idioms)]
// ************************
// * CV *******************
// ************************
#[derive(Default, Debug, Eq, PartialEq)]
pub struct Cv {
    id: Identity,
    tldr: String,
    skills: Vec<Skill>,
    experience: Vec<Experience>,
    education: Vec<Education>,
    about: String,
}

impl Cv {
    pub fn new(
        id: Identity,
        tldr: &str,
        skills: Vec<Skill>,
        experience: Vec<Experience>,
        education: Vec<Education>,
        about: &str,
    ) -> Self {
        Self {
            id,
            tldr: tldr.to_string(),
            skills,
            experience,
            education,
            about: about.to_string(),
        }
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct CvBuilder {
    id: Identity,
    tldr: String,
    skills: Vec<Skill>,
    exp: Vec<Experience>,
    edu: Vec<Education>,
    about: String,
}

impl CvBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn identity(mut self, id: Identity) -> Self {
        self.id = id;
        self
    }

    pub fn tldr(mut self, tldr: &str) -> Self {
        self.tldr = tldr.to_string();
        self
    }

    pub fn skill(mut self, skill: Skill) -> Self {
        self.skills.push(skill);
        self
    }

    pub fn experience(mut self, exp: Experience) -> Self {
        self.exp.push(exp);
        self
    }

    pub fn education(mut self, edu: Education) -> Self {
        self.edu.push(edu);
        self
    }

    pub fn about(mut self, about: &str) -> Self {
        self.about = about.to_string();
        self
    }

    pub fn build(self) -> Cv {
        Cv {
            id: self.id,
            tldr: self.tldr,
            skills: self.skills,
            experience: self.exp,
            education: self.edu,
            about: self.about,
        }
    }
}

// ************************
// * IDENTITY *************
// ************************
#[derive(Default, Debug, Eq, PartialEq)]
pub struct Identity {
    name: Name,
    contact: Contact,
    location: Location,
}

impl Identity {
    pub fn new(name: Name, contact: Contact, location: Location) -> Self {
        Self {
            name,
            contact,
            location,
        }
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct IdentityBuilder {
    name: Name,
    contact: Contact,
    location: Location,
}

impl IdentityBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: Name) -> Self {
        self.name = name;
        self
    }

    pub fn contact(mut self, contact: Contact) -> Self {
        self.contact = contact;
        self
    }

    pub fn location(mut self, location: Location) -> Self {
        self.location = location;
        self
    }

    pub fn build(self) -> Identity {
        Identity {
            name: self.name,
            contact: self.contact,
            location: self.location,
        }
    }
}

// ************************
// * NAME *****************
// ************************
#[derive(Default, Debug, Eq, PartialEq)]
pub struct Name {
    // conflicted about making these &'a str or &'static str or String...
    // probably should do 'a, but I'm not sure my CTRL+F is up to the task!!
    prefix: String,
    first: String,
    middle: String,
    last: String,
    suffix: String,
    alt: String,
}

impl Name {
    pub fn new(
        prefix: &str,
        first: &str,
        middle: &str,
        last: &str,
        suffix: &str,
        alt: &str,
    ) -> Self {
        Self {
            prefix: prefix.to_string(),
            first: first.to_string(),
            middle: middle.to_string(),
            last: last.to_string(),
            suffix: suffix.to_string(),
            alt: alt.to_string(),
        }
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct NameBuilder {
    p: String,
    f: String,
    m: String,
    l: String,
    s: String,
    a: String,
}

impl NameBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn prefix(mut self, i: &str) -> Self {
        self.p = i.to_string();
        self
    }

    pub fn first(mut self, i: &str) -> Self {
        self.f = i.to_string();
        self
    }

    pub fn middle(mut self, i: &str) -> Self {
        self.m = i.to_string();
        self
    }

    pub fn last(mut self, i: &str) -> Self {
        self.l = i.to_string();
        self
    }

    pub fn alt(mut self, i: &str) -> Self {
        self.a = i.to_string();
        self
    }

    pub fn build(self) -> Name {
        Name {
            prefix: self.p,
            first: self.f,
            middle: self.m,
            last: self.l,
            suffix: self.s,
            alt: self.a,
        }
    }
}

// ************************
// * EDUCATION ************
// ************************
#[derive(Default, Debug, Eq, PartialEq)]
pub struct Education {
    institution: Identity,
    dates: (String, String), // vec maybe; dunno, shouldn't risk it.
    subject: String,
    certificate: String,
    about: String,
}

impl Education {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_dates(&mut self, from: &str, to: &str) {
        self.dates = (from.to_string(), to.to_string());
    }

    pub fn set_subject(&mut self, subject: &str) {
        self.subject = subject.to_string();
    }

    pub fn set_certificate(&mut self, cert: &str) {
        self.certificate = cert.to_string();
    }

    pub fn set_about(&mut self, about: &str) {
        self.about = about.to_string();
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct EducationBuilder {
    id: Identity,
    dt: (String, String),
    sb: String,
    ct: String,
    ab: String,
}

impl EducationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn institution(mut self, id: Identity) -> Self {
        self.id = id;
        self
    }

    pub fn dates(mut self, from: &str, to: &str) -> Self {
        self.dt = (from.to_string(), to.to_string());
        self
    }

    pub fn subject(mut self, subject: &str) -> Self {
        self.sb = subject.to_string();
        self
    }

    pub fn certificate(mut self, cert: &str) -> Self {
        self.ct = cert.to_string();
        self
    }

    pub fn about(mut self, about: &str) -> Self {
        self.ab = about.to_string();
        self
    }

    pub fn build(self) -> Education {
        Education {
            institution: self.id,
            dates: self.dt,
            subject: self.sb,
            certificate: self.ct,
            about: self.ab,
        }
    }
}

// ************************
// * EXPERIENCE ***********
// ************************
#[derive(Default, Debug, Eq, PartialEq)]
pub struct Experience {
    institution: Identity,
    role: String,
    dates: (String, String),
    about: String,
}

impl Experience {
    pub fn new(institution: Identity, role: &str, from: &str, to: &str, about: &str) -> Self {
        Self {
            institution,
            role: role.to_string(),
            dates: (from.to_string(), to.to_string()),
            about: about.to_string(),
        }
    }

    pub fn set_institution(&mut self, institution: Identity) {
        self.institution = institution;
    }

    pub fn set_role(&mut self, role: &str) {
        self.role = role.to_string();
    }

    pub fn set_dates(&mut self, from: &str, to: &str) {
        self.dates = (from.to_string(), to.to_string());
    }

    pub fn set_about(&mut self, about: &str) {
        self.about = about.to_string();
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct ExperienceBuilder {
    id: Identity,
    rl: String,
    dt: (String, String),
    ab: String,
}

impl ExperienceBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn institution(mut self, id: Identity) -> Self {
        self.id = id;
        self
    }

    pub fn role(mut self, role: &str) -> Self {
        self.rl = role.to_string();
        self
    }

    pub fn dates(mut self, from: &str, to: &str) -> Self {
        self.dt = (from.to_string(), to.to_string());
        self
    }

    pub fn about(mut self, about: &str) -> Self {
        self.ab = about.to_string();
        self
    }

    pub fn build(self) -> Experience {
        Experience {
            institution: self.id,
            role: self.rl,
            dates: self.dt,
            about: self.ab,
        }
    }
}

// ************************
// * CONTACT **************
// ************************
#[derive(Default, Debug, Eq, PartialEq)]
pub struct Contact {
    phone: Vec<(String, String)>,
    email: Vec<(String, String)>,
    web: Vec<(String, String)>,
    chat: Vec<(String, String)>,
}

impl Contact {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_phone(&mut self, label: &str, phone: &str) {
        self.phone.push((label.to_string(), phone.to_string()));
    }

    pub fn add_email(&mut self, label: &str, email: &str) {
        self.email.push((label.to_string(), email.to_string()));
    }

    pub fn add_web(&mut self, label: &str, website: &str) {
        self.web.push((label.to_string(), website.to_string()));
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct ContactBuilder {
    ph: Vec<(String, String)>,
    em: Vec<(String, String)>,
    we: Vec<(String, String)>,
    ch: Vec<(String, String)>,
}

impl ContactBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn phone(mut self, label: &str, phone: &str) -> Self {
        self.ph.push((label.to_string(), phone.to_string()));
        self
    }

    pub fn email(mut self, label: &str, email: &str) -> Self {
        self.em.push((label.to_string(), email.to_string()));
        self
    }

    pub fn web(mut self, label: &str, web: &str) -> Self {
        self.we.push((label.to_string(), web.to_string()));
        self
    }

    pub fn chat(mut self, label: &str, chat: &str) -> Self {
        self.ch.push((label.to_string(), chat.to_string()));
        self
    }

    pub fn build(self) -> Contact {
        Contact {
            phone: self.ph,
            email: self.em,
            web: self.we,
            chat: self.ch,
        }
    }
}

// ************************
// * LOCATION *************
// ************************
#[derive(Default, Debug, Eq, PartialEq)]
pub struct Location {
    country: String,
    state: String,
    county: String,
    city: String,
    street: String,
    building: String,
    address: String,
    suit: String,
    postal_code: u32,
    gps_lat: String, //deg decimal
    gps_long: String,
}

impl Location {
    pub fn new_street_addr(
        address: &str,
        street: &str,
        city: &str,
        state: &str,
        postal_code: u32,
    ) -> Self {
        Self {
            address: address.to_string(),
            street: street.to_string(),
            city: city.to_string(),
            state: state.to_string(),
            postal_code,
            ..Default::default()
        }
    }

    pub fn new_apartment_address(
        address: &str,
        suit: &str,
        building: &str,
        street: &str,
        city: &str,
        state: &str,
        postal_code: u32,
    ) -> Self {
        Self {
            address: address.to_string(),
            suit: suit.to_string(),
            building: building.to_string(),
            street: street.to_string(),
            city: city.to_string(),
            state: state.to_string(),
            postal_code,
            ..Default::default()
        }
    }

    pub fn new_business_address(
        address: &str,
        suit: &str,
        building: &str,
        street: &str,
        city: &str,
        state: &str,
        country: &str,
    ) -> Self {
        Self {
            address: address.to_string(),
            suit: suit.to_string(),
            building: building.to_string(),
            street: street.to_string(),
            city: city.to_string(),
            state: state.to_string(),
            country: country.to_string(),
            ..Default::default()
        }
    }

    pub fn new_gps(gps_lat: &str, gps_long: &str) -> Self {
        Self {
            gps_lat: gps_lat.to_string(),
            gps_long: gps_long.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct LocationBuilder {
    cn: String,
    st: String,
    co: String,
    ci: String,
    rd: String,
    bu: String,
    ad: String,
    su: String,
    pc: u32,
    la: String,
    lo: String,
}

impl LocationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn country(mut self, input: &str) -> Self {
        self.cn = input.to_string();
        self
    }

    pub fn state(mut self, input: &str) -> Self {
        self.st = input.to_string();
        self
    }

    pub fn county(mut self, input: &str) -> Self {
        self.co = input.to_string();
        self
    }

    pub fn city(mut self, input: &str) -> Self {
        self.ci = input.to_string();
        self
    }

    pub fn street(mut self, input: &str) -> Self {
        self.rd = input.to_string();
        self
    }

    pub fn building(mut self, input: &str) -> Self {
        self.bu = input.to_string();
        self
    }

    pub fn address(mut self, input: &str) -> Self {
        self.ad = input.to_string();
        self
    }

    pub fn suit(mut self, input: &str) -> Self {
        self.su = input.to_string();
        self
    }

    pub fn postal_code(mut self, input: u32) -> Self {
        self.pc = input;
        self
    }

    pub fn gps_lat(mut self, input: &str) -> Self {
        self.la = input.to_string();
        self
    }
    pub fn gps_long(mut self, input: &str) -> Self {
        self.lo = input.to_string();
        self
    }

    pub fn gps(mut self, lat: &str, long: &str) -> Self {
        self.la = lat.to_string();
        self.lo = long.to_string();
        self
    }

    pub fn build(self) -> Location {
        Location {
            country: self.cn,
            state: self.st,
            county: self.co,
            city: self.ci,
            street: self.rd,
            building: self.bu,
            address: self.ad,
            suit: self.su,
            postal_code: self.pc,
            gps_lat: self.la,
            gps_long: self.lo,
        }
    }
}

// ************************
// * SKILL ****************
// ************************
#[derive(Default, Debug, Eq, PartialEq)]
pub struct Skill {
    skill: String,
    experience: u32,
    rank: u32,
    category: Vec<String>,
    sub_category: Vec<String>,
}

impl Skill {
    // cat-vomit-constructor
    pub fn new(s: &str, exp: u32, rank: u32, cat: &str, subcat: &str) -> Self {
        Skill {
            skill: s.to_string(),
            experience: exp,
            rank,
            category: vec![cat.to_string()],
            sub_category: vec![subcat.to_string()],
        }
    }

    fn add_category(&mut self, cat: &str) {
        self.category.push(cat.to_string());
    }

    pub fn add_subcategory(&mut self, subcat: &str) {
        self.category.push(subcat.to_string());
    }

    pub fn add_categories(&mut self, cat: &str, subcat: &str) {
        self.add_category(cat);
        self.add_subcategory(subcat);
    }

    pub fn rename(&mut self, skill: &str) {
        self.skill = skill.to_string();
    }
}

// nice pretty skill building type, SkillderBuilder
#[derive(Default, Debug, Eq, PartialEq)]
pub struct SkillBuilder {
    skill: String,
    experience: u32,
    rank: u32,
    category: Vec<String>,
    sub_category: Vec<String>,
}

impl SkillBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn skill(mut self, skill: &str) -> Self {
        self.skill = skill.to_string();
        self
    }

    pub fn exp(mut self, exp: u32) -> Self {
        self.experience = exp;
        self
    }

    pub fn rank(mut self, rank: u32) -> Self {
        self.rank = rank;
        self
    }

    pub fn assign_category(mut self, cat: &str) -> Self {
        self.category.push(cat.to_string());
        self
    }

    pub fn assign_subcat(mut self, subcat: &str) -> Self {
        self.sub_category.push(subcat.to_string());
        self
    }

    pub fn build(self) -> Skill {
        Skill {
            skill: self.skill,
            experience: self.experience,
            rank: self.rank,
            category: self.category,
            sub_category: self.sub_category,
        }
    }
}

// ************************
// * TEST TEST  TEST TEST *
// ************************
#[cfg(test)]
mod tests {
    use super::*;
    use std::any::type_name;

    fn type_of<T>(_: &T) -> &'static str {
        type_name::<T>()
    }

    #[test]
    fn it_works() {
        assert!(true)
    }

    #[test]
    #[should_panic]
    fn it_fails() {
        assert!(false)
    }

    #[test]
    fn skill_builder_sanity() {
        let a = SkillBuilder::new().build();
        let x = Skill::default();
        assert_eq!(type_of(&x), type_of(&a))
    }

    #[test]
    fn name_builder_sanity() {
        let a = NameBuilder::new().build();
        let x = Name::default();
        assert_eq!(type_of(&x), type_of(&a));
        assert_eq!(a, x)
    }

    #[test]
    fn location_builder_sanity() {
        let a = LocationBuilder::new().build();
        let x = Location::default();
        assert_eq!(type_of(&x), type_of(&a));
        assert_eq!(a, x)
    }

    #[test]
    fn experience_builder_sanity() {
        let a = ExperienceBuilder::new().build();
        let x = Experience::default();
        assert_eq!(type_of(&x), type_of(&a));
        assert_eq!(a, x)
    }

    #[test]
    fn education_builder_sanity() {
        let a = EducationBuilder::new().build();
        let x = Education::default();
        assert_eq!(type_of(&x), type_of(&a));
        assert_eq!(a, x)
    }

    #[test]
    fn contact_builder_sanity() {
        let a = ContactBuilder::new().build();
        let x = Contact::default();
        assert_eq!(type_of(&x), type_of(&a));
        assert_eq!(a, x)
    }
    #[test]
    fn cv_builder_sanity() {
        let a = CvBuilder::new().build();
        let x = Cv::default();
        assert_eq!(type_of(&x), type_of(&a));
        assert_eq!(a, x)
    }
}
