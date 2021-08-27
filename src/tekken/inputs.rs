use nom::IResult;

pub enum DirectionalInput {
    Neutral,
    Back,
    Forward,
    Down,
    Up,
    DownBack,
    DownForward,
    UpBack,
    UpForward,
}

pub enum FaceButtonInput {
    OneTwoThreeFour,
    OneTwoThree,
    OneTwoFour,
    OneThreeFour,
    TwoThreeFour,
    OneTwo,
    OneThree,
    OneFour,
    TwoThree,
    TwoFour,
    ThreeFour,
    One,
    Two,
    Three,
    Four,
    Neutral,
}

pub enum AtomicInput {
    Directional(DirectionalInput),
    FaceButton(FaceButtonInput),
}

pub enum InputModifier {
    HoldDirectionalInput,
    HoldFaceButtonInput,
    Fast,      // tilde
    JustFrame, // notated with a ":"
    None,
}

// TODO handle stance transitions
pub type MoveInput = Vec<(AtomicInput, InputModifier)>;

pub fn parse_directional_input(input: &str) -> IResult<&str, Option<DirectionalInput>> {
    named!(pub direction_tag<&str, &str>, alt!(
        tag!("db")
        | tag!("df")
        | tag!("ub")
        | tag!("uf")
        | tag!("n")
        | tag!("u")
        | tag!("d")
        | tag!("b")
        | tag!("f")
    ));

    let (i, raw_direction) = direction_tag(input)?;
    let direction = match raw_direction {
        "db" => Some(DirectionalInput::DownBack),
        "df" => Some(DirectionalInput::DownForward),
        "ub" => Some(DirectionalInput::UpBack),
        "uf" => Some(DirectionalInput::UpForward),
        "d" => Some(DirectionalInput::Down),
        "u" => Some(DirectionalInput::Up),
        "b" => Some(DirectionalInput::Back),
        "f" => Some(DirectionalInput::Forward),
        "n" => Some(DirectionalInput::Neutral),
        _ => None,
    };
    Ok((i, direction))
}

pub fn parse_facebutton_input(input: &str) -> IResult<&str, Option<FaceButtonInput>> {
    named!(pub facebutton_tag<&str, &str>, alt!(
        tag!("1+2+3+4")
        | tag!("1+2+3")
        | tag!("1+2+4")
        | tag!("1+3+4")
        | tag!("2+3+4")
        | tag!("1+2")
        | tag!("1+3")
        | tag!("1+4")
        | tag!("2+3")
        | tag!("2+4")
        | tag!("3+4")
        | tag!("1")
        | tag!("2")
        | tag!("3")
        | tag!("4")
    ));

    let (i, raw_facebutton) = facebutton_tag(input)?;
    let facebutton = match raw_facebutton {
        "1+2+3+4" => Some(FaceButtonInput::OneTwoThreeFour),
        "1+2+4" => Some(FaceButtonInput::OneTwoFour),
        "1+2+3" => Some(FaceButtonInput::OneTwoThree),
        "1+3+4" => Some(FaceButtonInput::OneThreeFour),
        "2+3+4" => Some(FaceButtonInput::TwoThreeFour),
        "1+2" => Some(FaceButtonInput::OneTwo),
        "1+3" => Some(FaceButtonInput::OneThree),
        "1+4" => Some(FaceButtonInput::OneFour),
        "2+3" => Some(FaceButtonInput::TwoThree),
        "2+4" => Some(FaceButtonInput::TwoFour),
        "3+4" => Some(FaceButtonInput::ThreeFour),
        "1" => Some(FaceButtonInput::One),
        "2" => Some(FaceButtonInput::Two),
        "3" => Some(FaceButtonInput::Three),
        "4" => Some(FaceButtonInput::Four),
        _ => None,
    };
    Ok((i, facebutton))
}

pub fn parse_input(input: &str) -> IResult<&str, Option<MoveInput>> {
    alt!(parse_facebutton_input | parse_directional_input)
}
