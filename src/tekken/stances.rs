use nom::IResult;

pub enum GroundedStance {
    FaceUpFeetAway,
    FaceUpFeetToward,
    FaceDownFeetAway,
    FaceDownFeetToward,
}

pub enum SidestepDirection {
    Left,
    Right,
    Either,
}

pub enum Stance {
    Rage,
    OpponentGrounded,
    Grounded(GroundedStance),
    Sidestepping(SidestepDirection),
    FullCrouch,
    WhileStanding,
    WhileRunning,
    Backturned,
    NoStance,
    // akuma
    DemonFlip,
    Teleport,
    // 2ds
    Jumping,
    // alisa
    Destructive,
    SingleBoot,
    DualBoot,
    // anna
    ChaosJudgement,
    // armor king
    ShadowStep,
    // asuka
    LegCutter,
    // bob
    ReverseBall,
    Ball,
    // claudio
    Starburst,
    // devil jin & kazumi
    Fly,
    // eddy
    Relaxed,
    HandstandPosition,
    // eliza
    MoonGlide,
    // feng
    ShiftingClouds,
    Kenpo,
    // ganryu
    Tachiai,
    Sit,
    // geese
    MaxMode,
    // gigass
    Golem,
    Goliath,
    // bryan
    Taunt,
    Sway,
    Fisherman,
    // heihachi
    Raijin,
    // hwoarang
    LeftFootForward,
    RightFootForward,
    LeftFlamingoStance,
    RightFlamingoStance,
    BackturnedLeftFootForward,
    BackturnedRightFootForward,
    BackturnedLeftFlamingoStance,
    BackturnedRightFlamingoStance,
    // jin
    Zenshin,
    CrouchingDemonStance,
    // josie
    SwitchStance,
    // julia
    ClockwiseEvasiveStance,
    // katarina
    Harrier,
    // kazuya
    Devilkin,
    // kuma/panda
    HuntingBearStance,
    Roll,
    // kunimitsu
    Katon,
    Setsu,
    // lars
    DynamicEntry,
    SilentEntry,
    // law
    DragonSignStance,
    DragonFakeStance,
    // lee
    HitmanStance,
    MistStep,
    // lei
    SnakeStance,
    PantherStance,
    DragonStance,
    DrunkenStance,
    TigerStance,
    CraneStance,
    TortoiseStance,
    PhoenixStance,
    PlayDeadRelaxed,
    KnockdownRelaxed,
    SlideRelaxed,
    FacedownRelaxed,
    KaiSheShen,
    // leo
    JinJiDuLi,
    FoBu,
    // leroy
    HermitStance,
    // lidia
    CatFootStanceI,
    CatFootStanceII,
    HeavenAndEarth,
    PouncingTigerStalkingWolf,
    // lucky chloe
    TwistLeft,
    TwistRight,
    Scoot,
    // marduk
    ValeTudo,
    // master raven
    Haze,
    // miguel
    Savage,
    // negan
    Intimidation,
    // shaheen
    Sneak,
    // steve
    Duck,
    Flicker,
    Peekaboo,
    LeftWeave,
    RightWeave,
    // xiaoyu
    ArtOfPhoenix,
    RainDanceStance,
    Hypnotist,
    // yoshimitsu
    NoSwordStance,
    Kincho,
    Meditation,
    ManjiDragonfly,
    Flea,
    IndianStance,
    // zafina
    Scarecrow,
    Mantis,
    Tarantula,
}

pub fn parse_stance(input: &str) -> IResult<&str, Stance> {
    named!(
        pub stance_tag<&str, &str>,
        alt!(
            // universal stances
            complete!(alt!(tag!("in rage") | tag!("rage")))
                | complete!(tag!("opponent grounded"))
                | complete!(tag!("SS"))
                | complete!(tag!("SSR"))
                | complete!(tag!("SSL"))
                | complete!(tag!("FUFA"))
                | complete!(tag!("FUFT"))
                | complete!(tag!("FDFA"))
                | complete!(tag!("FDFT"))
                | complete!(tag!("FC"))
                | complete!(tag!("WS"))
                | complete!(tag!("WR"))
                | complete!(tag!("BT"))
                // akuma stances
                | complete!(tag!("DFLIP"))
                | complete!(tag!("TPORT"))
                // akuma, geese, eliza
                | complete!(tag!("j."))
                // alisa stances
                | complete!(tag!("DES"))
                | complete!(tag!("SBT"))
                | complete!(tag!("DBT"))
                // anna stances
                | complete!(tag!("CJM"))
                // armor king stances
                | complete!(tag!("SHS"))
                // asuka stances
                | complete!(tag!("LCT"))
                // bob stances
                | complete!(tag!("REV BAL"))
                | complete!(tag!("BAL"))
                // bryan stances
                | complete!(tag!("TNT"))
                | complete!(tag!("SWAY"))
                | complete!(tag!("GUP"))
                // claudio stances
                | complete!(tag!("STB"))
                // devil jin stances
                | complete!(tag!("FLY"))
                // eddy stances
                | complete!(tag!("RLX"))
                | complete!(tag!("HSP"))
                // eliza stances
                | complete!(tag!("MG"))
                // feng stances
                | complete!(tag!("STC"))
                | complete!(tag!("KNP"))
                // ganryu stances
                | complete!(tag!("TCH"))
                | complete!(tag!("SIT"))
                // geese stances
                | complete!(tag!("MAX"))
                // gigass stances
                | complete!(tag!("GOL"))
                | complete!(tag!("GS"))
                // heihachi stances
                | complete!(tag!("RAI"))
                // hwoarang stances
                | complete!(tag!("RFF"))
                | complete!(tag!("LFF"))
                | complete!(tag!("LFS"))
                | complete!(tag!("RFS"))
                | complete!(tag!("LFF BT"))
                | complete!(tag!("RFF BT"))
                | complete!(tag!("LFS BT"))
                | complete!(tag!("RFS BT"))
                // jin stances
                | complete!(tag!("ZEN"))
                | complete!(tag!("CDS"))
                // josie stances
                | complete!(tag!("SWS"))
                | complete!(tag!("SWB"))
                // julia stances
                | complete!(tag!("CES"))
                // katarina stances
                | complete!(tag!("HAR"))
                // kazuya stances
                | complete!(tag!("DVK"))
                // kuma/panda stances
                | complete!(tag!("HBS"))
                | complete!(tag!("ROL"))
                // kunimitsu stances
                | complete!(tag!("KAT"))
                | complete!(tag!("SET"))
                // lars stances
                | complete!(tag!("SEN"))
                | complete!(tag!("DEN"))
                // law stances
                | complete!(tag!("DSS"))
                | complete!(alt!(tag!("DFS") | tag!("TFS")))
                // lee stances
                | complete!(tag!("HMS"))
                | complete!(tag!("MS"))
                // lei stances
                | complete!(tag!("SNA"))
                | complete!(tag!("DRA"))
                | complete!(tag!("PAN"))
                | complete!(tag!("TGR"))
                | complete!(tag!("CRA"))
                | complete!(tag!("DRU"))
                | complete!(tag!("KSS"))
                | complete!(tag!("FCD RLX"))
                | complete!(tag!("SLD RLX"))
                | complete!(tag!("KND RLX"))
                | complete!(tag!("PLD RLX"))
                | complete!(tag!("PHX"))
                // leo stances
                | complete!(tag!("BOK"))
                | complete!(tag!("KNK"))
                // leroy stances
                | complete!(tag!("HRM"))
                // lidia stances
                | complete!(alt!(tag!("CF0") | tag!("CFO")))
                | complete!(tag!("CFT"))
                | complete!(tag!("HAE"))
                | complete!(tag!("TAW"))
                // lucky chloe stances
                | complete!(tag!("TWISTL"))
                | complete!(tag!("TWISTR"))
                | complete!(tag!("SCOOT"))
                // marduk stances
                | complete!(tag!("VTS"))
                // master raven stances
                | complete!(tag!("HAZ"))
                // miguel stances
                | complete!(tag!("SAV"))
                // negan stances
                | complete!(tag!("INT"))
                // shaheen stances
                | complete!(tag!("SNK"))
                // steve stances
                | complete!(tag!("DUK"))
                | complete!(tag!("FLK"))
                | complete!(tag!("PAB"))
                | complete!(tag!("LWV"))
                | complete!(tag!("RWV"))
                | complete!(tag!("SWY"))
                // xiaoyu stances
                | complete!(tag!("AOP"))
                | complete!(tag!("RDS"))
                | complete!(tag!("HYP"))
                // yoshimitsu stances
                | complete!(tag!("KIN"))
                | complete!(tag!("DGF"))
                | complete!(tag!("MED"))
                | complete!(tag!("IND"))
                | complete!(tag!("FLE"))
                | complete!(tag!("NSS"))
                // zafina stances
                | complete!(tag!("SCR"))
                | complete!(tag!("MNT"))
                | complete!(tag!("TRT"))
        )
    );
    let (i, stance) = stance_tag(input)?;
    let parsed_stance = match stance {
        "in rage" | "rage" => Stance::Rage,
        "opponent grounded" => Stance::OpponentGrounded,
        "SS" => Stance::Sidestepping(SidestepDirection::Either),
        "SSR" => Stance::Sidestepping(SidestepDirection::Right),
        "SSL" => Stance::Sidestepping(SidestepDirection::Left),
        "FUFT" => Stance::Grounded(GroundedStance::FaceUpFeetToward),
        "FUFA" => Stance::Grounded(GroundedStance::FaceUpFeetAway),
        "FDFA" => Stance::Grounded(GroundedStance::FaceDownFeetAway),
        "FDFT" => Stance::Grounded(GroundedStance::FaceDownFeetToward),
        "FC" => Stance::FullCrouch,
        "WS" => Stance::WhileStanding,
        "WR" => Stance::WhileRunning,
        "BT" => Stance::Backturned,
        // akuma
        "DFLIP" => Stance::DemonFlip,
        "TPORT" => Stance::Teleport,
        "j." => Stance::Jumping,
        // alisa
        "DES" => Stance::Destructive,
        "SBT" => Stance::SingleBoot,
        "DBT" => Stance::DualBoot,
        // anna
        "CJM" => Stance::ChaosJudgement,
        // armor king
        "SHS" => Stance::ShadowStep,
        // asuka
        "LCT" => Stance::LegCutter,
        // bob
        "BAL" => Stance::Ball,
        "REV BAL" => Stance::ReverseBall,
        // bryan
        "TNT" => Stance::Taunt,
        "SWAY" | "SWY" => Stance::Sway,
        "GUP" => Stance::Fisherman,
        // claudio
        "STB" => Stance::Starburst,
        // devil jin
        "FLY" => Stance::Fly,
        // eddy
        "RLX" => Stance::Relaxed,
        "HSP" => Stance::HandstandPosition,
        // eliza
        "MG" => Stance::MoonGlide,
        // feng
        "STC" => Stance::ShiftingClouds,
        "KNP" => Stance::Kenpo,
        // ganryu
        "TCH" => Stance::Tachiai,
        "SIT" => Stance::Sit,
        // geese
        "MAX" => Stance::MaxMode,
        // gigass
        "GS" => Stance::Goliath,
        "GOL" => Stance::Golem,
        // hwoarang
        "LFF" => Stance::LeftFootForward,
        "RFF" => Stance::RightFootForward,
        "LFS" => Stance::LeftFlamingoStance,
        "RFS" => Stance::RightFlamingoStance,
        "LFF BT" => Stance::BackturnedLeftFootForward,
        "RFF BT" => Stance::BackturnedRightFootForward,
        "LFS BT" => Stance::BackturnedLeftFlamingoStance,
        "RFS BT" => Stance::BackturnedRightFlamingoStance,
        // jin
        "ZEN" => Stance::Zenshin,
        "CDS" => Stance::CrouchingDemonStance,
        // josie
        "SWS" => Stance::SwitchStance,
        // julia
        "CES" => Stance::ClockwiseEvasiveStance,
        // katarina
        "HAR" => Stance::Harrier,
        // kazuya
        "DVK" => Stance::Devilkin,
        // kuma/panda
        "HBS" => Stance::HuntingBearStance,
        "ROL" => Stance::Roll,
        // kunimitsu
        "KAT" => Stance::Katon,
        "SET" => Stance::Setsu,
        // lars
        "DEN" => Stance::DynamicEntry,
        "SEN" => Stance::SilentEntry,
        // law
        "DSS" => Stance::DragonSignStance,
        "TFS" | "DFS" => Stance::DragonFakeStance,
        // lee
        "HMS" => Stance::HitmanStance,
        "MS" => Stance::MistStep,
        // lei
        "SNA" => Stance::SnakeStance,
        "DRA" => Stance::DragonStance,
        "PAN" => Stance::PantherStance,
        "TGR" => Stance::TigerStance,
        "CRA" => Stance::CraneStance,
        "DRU" => Stance::DrunkenStance,
        "KSS" => Stance::KaiSheShen,
        "PHX" => Stance::PhoenixStance,
        "FCD RLX" => Stance::FacedownRelaxed,
        "SLD RLX" => Stance::SlideRelaxed,
        "KND RLX" => Stance::KnockdownRelaxed,
        "PLD RLX" => Stance::PlayDeadRelaxed,
        // leo
        "BOK" => Stance::JinJiDuLi,
        "KNK" => Stance::FoBu,
        // leroy
        "HRM" => Stance::HermitStance,
        // lidia
        "CF0" | "CFO" => Stance::CatFootStanceI,
        "CFT" => Stance::CatFootStanceII,
        "HAE" => Stance::HeavenAndEarth,
        "TAW" => Stance::PouncingTigerStalkingWolf,
        // lucky chloe
        "TWISTL" => Stance::TwistLeft,
        "TWISTR" => Stance::TwistRight,
        "SCOOT" => Stance::Scoot,
        // marduk
        "VTS" => Stance::ValeTudo,
        // master raven
        "HAZ" => Stance::Haze,
        // miguel
        "SAV" => Stance::Savage,
        // negan
        "INT" => Stance::Intimidation,
        // shaheen
        "SNK" => Stance::Sneak,
        // steve
        "DUK" => Stance::Duck,
        "PAB" => Stance::Peekaboo,
        "LWV" => Stance::LeftWeave,
        "RWV" => Stance::RightWeave,
        "FLK" => Stance::Flicker,
        // xiaoyu
        "AOP" => Stance::ArtOfPhoenix,
        "RDS" => Stance::RainDanceStance,
        "HYP" => Stance::Hypnotist,
        // yoshi
        "KIN" => Stance::Kincho,
        "DGF" => Stance::ManjiDragonfly,
        "MED" => Stance::Meditation,
        "IND" => Stance::IndianStance,
        "FLE" => Stance::Flea,
        "NSS" => Stance::NoSwordStance,
        // zafina
        "SCR" => Stance::Scarecrow,
        "MNT" => Stance::Mantis,
        "TRT" => Stance::Tarantula,
        _ => Stance::NoStance,
    };

    Ok((i, parsed_stance))
}
