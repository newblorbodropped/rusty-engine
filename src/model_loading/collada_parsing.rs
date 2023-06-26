#[derive(PartialEq)]
#[derive(Debug)]
pub enum TagParameter<'a> {
    ParameterString(&'a str, &'a str),
    ParameterInt(&'a str, i32),
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Collada<'a>{
    ColladaNone,
    ColladaHeader(Box<Collada<'a>>),
    ColladaString(&'a str),
    ColladaFloats(Vec<f32>),
    ColladaTag(&'a str, Vec<TagParameter<'a>>, Box<Vec<Collada<'a>>>),
    ColladaTagClosed(&'a str, Vec<TagParameter<'a>>),
}

pub trait Parser<T, I> {
    fn parse(&mut self, _: I) -> Option<(I, T)>;

    fn map<U, G>(self, g: G) -> Map<Self, G, T>
    where G: FnMut(T) -> U, Self: core::marker::Sized{
        Map {
            f: self,
            g: g,
            phantom: core::marker::PhantomData
        }
    }
    
    fn many(self) -> Many<Self>
    where Self: core::marker::Sized {
        Many {f: self}
    }

    fn many_delim<U,G>(self, g: G) -> ManyDelim<Self, G, U>
    where G: Parser<U,I>, Self: core::marker::Sized{
        ManyDelim {
            f: self,
            g: g,
            phantom: core::marker::PhantomData
        }
    }

    fn and<U,G>(self, g: G) -> And<Self, G>
    where G: Parser<U, I>, Self: core::marker::Sized {
        And {
            f: self,
            g: g
        }
    }

    fn or<G>(self, g: G) -> Or<Self, G>
    where G: Parser<T, I>, Self: core::marker::Sized {
        Or {
            f: self,
            g: g
        }
    }

    fn maybe(self) -> Maybe<Self>
    where Self: core::marker::Sized {
        Maybe{f: self}
    }
}

pub struct Map<F, G, T> {
    f: F,
    g: G,
    phantom: std::marker::PhantomData<T> 
}

pub struct Many<F> {
    f: F
}

pub struct ManyDelim<F, G, U> {
    f: F,
    g: G,
    phantom: std::marker::PhantomData<U>
}

pub struct And<F, G> {
    f: F,
    g: G
}

pub struct Or<F, G> {
    f: F,
    g: G
}

pub struct Maybe<F> {
    f: F
}

impl<T,I,U,F,G> Parser<U,I> for Map<F,G,T>
where F: Parser<T, I>, G: FnMut(T) -> U {
    fn parse(&mut self, i: I) -> Option<(I, U)>{
        match self.f.parse(i) {
            Some((rest, x)) => Some((rest, (self.g)(x))),
            None => None
        }
    }
}

impl<T,I: Clone,F> Parser<Vec<T>, I> for Many<F>
where F: Parser<T,I> {
    fn parse(&mut self, mut i: I) -> Option<(I, Vec<T>)> {
        match self.f.parse(i.clone()) {
            None => None,
            Some((rest, x)) => {
                let mut res = vec![];
                res.push(x);
                i = rest;

                loop {
                    match self.f.parse(i.clone()) {
                        None => { return Some((i, res)); },
                        Some((rest, x)) => {
                            i = rest;
                            res.push(x);
                        }
                    }
                }
            }
        }
    }
}

impl<T, I: Clone,U,G, F> Parser<Vec<T>, I> for ManyDelim<F,G,U>
where F: Parser<T,I>, G: Parser<U,I> {
    fn parse(&mut self, mut i: I) -> Option<(I, Vec<T>)> {
        match self.f.parse(i.clone()) {
            None => None,
            Some((rest, x)) => {
                let mut res = vec![];
                res.push(x);
                i = rest;

                loop {
                    match self.g.parse(i.clone()) {
                        None => {
                            return Some((i, res));
                        },
                        Some((rest, _del)) => {
                            i = rest;
                            match self.f.parse(i.clone()) {
                                None => {
                                    return Some((i, res));
                                },
                                Some((rest, x)) => {
                                    i = rest;
                                    res.push(x);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<T,I,U,F,G> Parser<(T,U),I> for And<F,G>
where F: Parser<T,I>, G: Parser<U,I> {
    fn parse(&mut self, i: I) -> Option<(I, (T,U))>{
        match self.f.parse(i) {
            None => None, 
            Some((rest1, x)) => match self.g.parse(rest1) {
                    None => None,
                    Some((rest2, y)) => Some((rest2, (x,y)))
                }
        }
    }
}

impl<T,I: Clone,F,G> Parser<T,I> for Or<F,G>
where F: Parser<T,I>, G: Parser<T,I> {
    fn parse(&mut self, i: I) -> Option<(I, T)>{
        match self.f.parse(i.clone()) {
            Some((rest1, x)) => Some((rest1, x)),
            None => match self.g.parse(i.clone()) {
                Some((rest2, y)) => Some((rest2, y)),
                None => None
            }
        }
    }
}

impl<'a, T, I: Clone, F> Parser<T, I> for Maybe<F>
where F: Parser<T,I>, T: std::default::Default, I: {
    fn parse(&mut self, i: I) -> Option<(I, T)> {
        match self.f.parse(i.clone()) {
            None => Some((i, T::default())),
            Some(x) => Some(x)
        }
    }
}

impl<'a, T, I, F> Parser<T, I> for F
where F: FnMut(I) -> Option<(I, T)> + 'a{
    fn parse(&mut self, i: I) -> Option<(I, T)> {
        self(i)
    }
}

fn parse_until<'a>(c: char) -> impl Parser<&'a str, &'a str> {
    move |input: &'a str| {
        match input.find(c) {
            None => Some((input, "")),
            Some(n) => Some((&input[n..], &input[0..n]))
        }
    }
}

fn parse_until_l<'a>(cs: Vec<char>) -> impl Parser<&'a str, &'a str> {
    move |input: &'a str| {
        let mut x: Option<usize> = None;
        for c in cs.iter() {
            match input.find(*c) {
                None => {},
                Some(n) => {
                    match x {
                        None => { x = Some(n); },
                        Some(m) => {
                            if n < m {
                                x = Some(n);
                            }
                        }
                    }
                }
            }
        }

        match x {
            None => Some((input, "")),
            Some(n) => Some((&input[n..], &input[0..n]))
        }
    }
}

fn parse_ws<'a: 'b, 'b>() -> impl Parser<(), &'a str> + 'b {
    move |input: &'a str| {
        match input.chars().next() {
            None => None,
            Some(c) => {
                if c.is_whitespace() {
                    Some((input.trim_start(), ()))                   
                } else {
                    None
                }
            }
        }
    }
}

fn parse_token<'a: 'b, 'b>(token: &'b str) -> impl Parser<&str, &'a str> + 'b {
    move |input: &'a str| {
        match input.strip_prefix(token) {
            Some(rest) => Some((rest, &input[..token.len()])),
            None => None
        }         
    }
}

fn parse_lookahead<'a: 'b, 'b>(token: &'b str) -> impl Parser<(), &'a str> + 'b {
    move |input: &'a str| {
        match input.strip_prefix(token) {
            Some(_) => Some((input, ())),
            None => None
        }
    }
}

fn parse_digit<'a: 'b, 'b>() -> impl Parser<u32 , &'a str> + 'b{
    move |input: &'a str| {
        if let Some(c) = input.chars().next() {
            const RADIX: u32 = 10;
            match c.to_digit(RADIX) {
                Some(x) => Some((&input[1..input.len()], x)),
                None => None
            }
        } else {
            None
        }
    }
}

fn parse_integer<'a>() -> impl Parser<i32, &'a str> {
    move |input: &'a str| {
        (parse_token("-").and(
            parse_digit().many()
        ).map(
            |t: (&str, Vec<u32>)| {
                let mut res: i32 = 0;
                let mut digits = t.1;
                let mut i: u32 = 0;
                while let Some(k) = digits.pop() {
                    res += (k as i32)*(i32::pow(10, i));
                    i += 1;
                }
                (-1) * res
            }
        )).or(
            (parse_digit().many()).map(
                |t: Vec<u32>| {
                    let mut res: i32 = 0;
                    let mut digits = t;
                    let mut i: u32 = 0;
                    while let Some(k) = digits.pop() {
                        res += (k as i32)*(i32::pow(10, i));
                        i += 1;
                    }
                    res
                }
            )
        ).parse(input)
    }
}

fn parse_float<'a>() -> impl Parser<f32, &'a str> {
    move |input: &'a str| {
        ((parse_digit().many()).and(
            parse_token(".").and(
                parse_digit().many()
            )
        ).map(
            |t: (Vec<u32>, (&str, Vec<u32>))| {
                let mut res: f32 = 0.0;
                let mut whole = t.0;
                let mut frac = t.1.1;
                let mut i: i32 = 0;
                let mut j: i32 = (-1) * (frac.len() as i32);

                while let Some(k) = whole.pop() {
                    res += (k as f32)*(f32::powi(10.0, i));
                    i += 1;
                }

                while let Some(k) = frac.pop() {
                    res += (k as f32)*(f32::powi(10.0, j));
                    j += 1;
                }

                res
            }
        )).or(
            (parse_digit().many()).map(
                |t: Vec<u32>| {
                    let mut res: f32 = 0.0;
                    let mut digits = t;
                    let mut i: i32 = 0;
                    while let Some(k) = digits.pop() {
                        res += (k as f32)*(f32::powi(10.0, i));
                        i += 1;
                    }
                    res
                }
            )
        ).parse(input)
    }
}

fn parse_scientific<'a>() -> impl Parser<f32, &'a str> {
    move |input: &'a str| {
        (parse_token("-").and(
            parse_float()
        ).and(
            parse_token("e")
        ).and(
            parse_integer()
        ).map(
            |t: (((&str, f32),&str),i32)| {
                let mnt = t.0.0.1;
                let exp = t.1;
                (-1.0) * mnt * f32::powi(10.0, exp)
            }
        )).or(
            parse_float().and(
                parse_token("e")
            ).and(
                parse_integer()
            ).map(
                |t: ((f32, &str), i32)| {
                    let mnt = t.0.0;
                    let exp = t.1;
                    mnt * f32::powi(10.0, exp)
                }
            )
        ).or(
            parse_token("-").and(
                parse_float()
            ).map(
                |t: (&str, f32)| {
                    (-1.0) * t.1
                }
            )
        ).or(
            parse_float()
        ).parse(input)
    }
}

fn collada_header_p<'a>() -> impl Parser<Collada<'a>, &'a str> {
    move |input: &'a str| {
        parse_token("<?").and(
            parse_until('?')
        ).and(
            parse_token("?>")
        ).and(
            parse_ws().maybe()
        ).and(
            collada_p()
        ).map(
            |t: ((((&str, &str),&str), ()), Collada)| {
                Collada::ColladaHeader(Box::new(t.1))
            }
        ).parse(input)
    }
}

fn collada_string_p<'a>() -> impl Parser<Collada<'a>, &'a str> {
    move |input: &'a str| {
        parse_until('<').map(
            |t: &str| {
                if t.is_empty() {
                    Collada::ColladaNone
                } else {
                    Collada::ColladaString(t)
                }
            }
        ).parse(input)
    }
}

fn collada_floats_p<'a>() -> impl Parser<Collada<'a>, &'a str> {
    move |input: &'a str| {
        parse_scientific().many_delim(parse_ws()).and(parse_lookahead("<")).map(
            |(t, _): (Vec<f32>, ())| {
                Collada::ColladaFloats(t)
            }
        ).parse(input)
    }
}

fn tag_parameter_int_p<'a>() -> impl Parser<TagParameter<'a>, &'a str> {
    move |input: &'a str| {
        parse_until('=').and(
            parse_token("=\"")
        ).and(
            parse_integer()
        ).and(
            parse_token("\"")
        ).map(
            |t: (((&str, &str), i32), &str)| {
                TagParameter::ParameterInt(t.0.0.0, t.0.1)
            }
        ).parse(input)
    }
}

fn tag_parameter_str_p<'a>() -> impl Parser<TagParameter<'a>, &'a str> {
    move |input: &'a str| {
        parse_until('=').and(
            parse_token("=\"")
        ).and(
            parse_until('"')
        ).and(
            parse_token("\"")
        ).map(
            |t: (((&str, &str), &str), &str)| {
                TagParameter::ParameterString(t.0.0.0, t.0.1)
            }
        ).parse(input)
    }
}

fn tag_parameter_p<'a>() -> impl Parser<TagParameter<'a>, &'a str> {
    move |input: &'a str| {
        match tag_parameter_int_p().or(tag_parameter_str_p()).parse(input) {
            None => None,
            Some((rest, TagParameter::ParameterString(name, content))) => {
                if name.contains('/') || name.contains('>') || name.contains('<') {
                    None
                } else {
                    Some((rest, TagParameter::ParameterString(name, content)))
                }
            },
            Some((rest, TagParameter::ParameterInt(name, content))) => {
                if name.contains('/') || name.contains('>') || name.contains('<') {
                    None
                } else {
                    Some((rest, TagParameter::ParameterInt(name, content)))
                }
            }
        }
    }
}

fn tag_identifier_p<'a>() -> impl Parser<&'a str, &'a str> {
    move |input: &'a str| {
        if let Some((rest, result)) = parse_until_l(vec!['>', ' ']).parse(input) {
            if result.contains('/') {
                None
            } else {
                Some((rest, result))
            }
        } else {
            None
        }
    }
}

fn tag_identifier_closed_p<'a>() -> impl Parser<&'a str, &'a str> {
    move |input: &'a str| {
        if let Some((rest, result)) = parse_until_l(vec!['/', ' ']).parse(input) {
            if result.contains('/') {
                None
            } else {
                Some((rest, result))
            }
        } else {
            None
        }
    }
}

fn collada_tag_p<'a>() -> impl Parser<Collada<'a>, &'a str> {
    move |input: &'a str| {
        parse_token("<").and(
            tag_identifier_p().and(
                parse_ws().maybe().and(
                    tag_parameter_p().many_delim(parse_ws()).maybe()
                )
            )
        ).and(
            parse_token(">")
        ).and(
            parse_ws().maybe().and(
                (collada_p().many_delim(parse_ws().maybe())).or(
                    collada_floats_p().map(|x| {vec![x]}).or(
                        collada_string_p().map(|x| {vec![x]})
                    )
                )
            ).and(
                parse_ws().maybe()
            )
        ).and(
            parse_token("</")
        ).and(
            parse_until('>')
        ).and(
            parse_token(">")
        ).map(
            |((((((_,(name,(_,params))), _), (((), content), ())),_),_,),_)| {
                Collada::ColladaTag(name, params, Box::new(content))
            }
        ).parse(input)
    }
}

fn collada_tag_closed_p<'a>() -> impl Parser<Collada<'a>, &'a str> {
    move |input: &'a str| {
        parse_token("<").and(
            tag_identifier_closed_p().and(
                parse_ws().maybe().and(
                    tag_parameter_p().many_delim(parse_ws()).maybe()
                )
            )
        ).and(
            parse_token("/>")
        ).map(
            |((_,(name,(_,params))), _)| {
                Collada::ColladaTagClosed(name, params)
            }
        ).parse(input)
    }
}

pub fn collada_p<'a>() -> impl Parser<Collada<'a>, &'a str> {
    move |input: &'a str| {
        collada_header_p().or(
            collada_tag_p()
        ).or(
            collada_tag_closed_p()
        ).parse(input)
    }
}

#[test]
fn parsing_a_digit_test() {
    assert_eq!(parse_digit().parse("123"), Some(("23", 1)));
}

#[test]
fn parsing_squares_test() {
    assert_eq!(parse_digit().map(|x: u32| {x * x}).parse("23asd"), Some(("3asd", 4)));
}

#[test]
fn parsing_many_digits_test() {
    assert_eq!(parse_digit().many().parse("123abc"), Some(("abc", vec![1, 2, 3])));
}

#[test]
fn parse_and_test() {
    assert_eq!(parse_digit().and(parse_token("hello")).parse("8hello_asf"), Some(("_asf", (8, "hello"))));
    assert_eq!((parse_digit().many()).and(parse_token("_")).parse("123_rest"),
               Some(("rest",(vec![1,2,3], "_"))));
    assert_eq!(parse_digit()
               .and(parse_digit())
               .parse("12345"),
               Some(("345", (1,2))));
}

#[test]
fn parse_or_test() {
    assert_eq!(parse_token("false").or(parse_token("true")).parse("true rest"), Some((" rest", "true")));
    assert_eq!(parse_token("false").or(parse_token("true")).parse("false rest"), Some((" rest", "false")));
    assert_eq!(parse_token("false").or(parse_token("true")).parse("troe"), None);
}

#[test]
fn parse_floats_test() {
    assert_eq!(parse_float().parse("12.623rest"), Some(("rest", 12.623001)));
    assert_eq!(parse_float().parse("12"), Some(("", 12.0)));
}

#[test]
fn parse_integer_test() {
    assert_eq!(parse_integer().parse("21 jump street"), Some((" jump street", 21)));
}

#[test]
fn parse_scientific_test() {
    assert_eq!(parse_scientific().parse("-1.234e-2"), Some(("", -0.012339999)));
}

#[test]
fn parse_ws_test() {
    assert_eq!(parse_ws().parse("     rest"), Some(("rest",())));
    assert_eq!(parse_ws().parse("nowhitespace"), None);
    assert_eq!(parse_ws().parse("no whitespace start"), None);
}

#[test]
fn parse_collada_floats_test() {
    assert_eq!(collada_floats_p().parse(
        "-1 -1 1 -1 1<"
    ), Some(("<", Collada::ColladaFloats(vec![-1.0, -1.0, 1.0, -1.0, 1.0]))));
}

#[test]
fn parse_until_test() {
    assert_eq!(parse_until('?').parse("asdf?rest"), Some(("?rest", "asdf")));
    assert_eq!(parse_until(' ').parse("test/>"), Some(("test/>", "")));
}

#[test]
fn parse_maybe_test() {
    assert_eq!(parse_token("hello").maybe().parse("helloworld"), Some(("world", "hello")));
    assert_eq!(parse_token("hello").maybe().parse("dontfailpls"), Some(("dontfailpls", "")));
}

#[test]
fn parse_until_l_test() {
    assert_eq!(parse_until_l(vec!['l', 'w', 'o']).parse("hello world"), Some(("llo world", "he")));
    assert_eq!(parse_until_l(vec!['a', 'b', 'c']).parse("hello world"), Some(("hello world", "")));
}

#[test]
fn collada_parse_test1() {
    let res : Collada = Collada::ColladaHeader(
        Box::new(Collada::ColladaTag(
            "test",
            vec![TagParameter::ParameterString("hello", "world")],
            Box::new(vec![Collada::ColladaString("content")])
        ))
    );

    assert_eq!(collada_p().parse("<?header?><test hello=\"world\">content</test>"), Some(("",res)));
}

#[test]
fn collada_parse_test2() {
    let res : Collada = Collada::ColladaFloats(vec![1.2, 2.0, 3.4]);
    assert_eq!(collada_floats_p().parse("1.2 2 3.4<"), Some(("<", res)));
}

#[test]
fn collada_parse_test3() {
    let res : Collada = Collada::ColladaTag(
        "test",
        vec![TagParameter::ParameterString("hello", "world")],
        Box::new(vec![Collada::ColladaString("content")])
    );
    assert_eq!(collada_p().parse("<test hello=\"world\">content</test>"), Some(("", res)));  
}

#[test]
fn collada_parse_test4() {
    let res : Collada = Collada::ColladaTagClosed(
        "test",
        vec![]
    );
    assert_eq!(collada_tag_closed_p().parse("<test/>"), Some(("", res)));
}

#[test]
fn collada_parse_test5() {
    let test_string : &str = "<created>2022-08-03T14:30:24</created>";
    println!("{:#?}", collada_p().parse(test_string));
    assert!(matches!(collada_p().parse(test_string), Some(_x)));
}

#[test]
fn collada_parse_test6() {
    let test_string : &str = "<source id=\"Cube_001-mesh-positions\"><float_array id=\"Cube_001-mesh-positions-array\" count=\"24\">-1 -1 -1 -1 -1 1 -1 1 -1 -1 1 1 1 -1 -1 1 -1 1 1 1 -1 1 1 1</float_array><technique_common><accessor source=\"#Cube_001-mesh-positions-array\" count=\"8\" stride=\"3\"><param name=\"X\" type=\"float\"/><param name=\"Y\" type=\"float\"/><param name=\"Z\" type=\"float\"/></accessor></technique_common></source>";
    println!("{:#?}", collada_p().parse(test_string));
    assert!(matches!(collada_p().parse(test_string), Some(_x)));
}

#[test]
fn test_whole_file1() {
    let cube : String = std::fs::read_to_string("resources/collada/cube.dae").unwrap();
    assert!(matches!(collada_p().parse(&cube[..]), Some(_x)));
}

#[test]
fn test_whole_file2() {
    let cones : String = std::fs::read_to_string("resources/collada/cones.dae").unwrap();
    assert!(matches!(collada_p().parse(&cones[..]), Some(_x)));
}

#[test]
fn test_vertices() {
    let test_string = "<float_array id=\"Cube_001-mesh-positions-array\" count=\"24\">-1 -1 -1 -1 -1 1 -1 1 -1 -1 1 1 1 -1 -1 1 -1 1 1 1 -1 1 1 1</float_array>";
    let desired_res = Collada::ColladaTag(
        "float_array",
        vec![
            TagParameter::ParameterString("id", "Cube_001-mesh-positions-array"),
            TagParameter::ParameterInt("count", 24)
        ],
        Box::new(vec![
            Collada::ColladaFloats(vec![-1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0])
        ])
    );
    assert_eq!(collada_p().parse(test_string), Some(("",desired_res)));
}
