use std::str::FromStr;

use hexf_parse::{parse_hexf32, parse_hexf64};

fn hexf32(s: &str) -> f32 {
    parse_hexf32(s, false).unwrap()
}

fn hexf64(s: &str) -> f64 {
    parse_hexf64(s, false).unwrap()
}

macro_rules! check {
    ($ty:ty, $s:expr) => {{
        check!($ty, stringify!($s), $s)
    }};
    ($ty:ident, $s:expr, inf) => {{
        check!($ty, $s, core::$ty::INFINITY)
    }};
    ($ty:ident, $s:expr, neg_inf) => {{
        check!($ty, $s, core::$ty::NEG_INFINITY)
    }};
    ($ty:ty, $s:expr, $e:expr) => {{
        let s = $s.as_bytes();
        let expected: $ty = $e;
        let result = fast_float::parse::<$ty, _>(s).unwrap();
        assert_eq!(result, expected);
        let lex = lexical_core::parse::<$ty>(s).unwrap();
        assert_eq!(result, lex);
        let std = <$ty>::from_str($s);
        if let Ok(std) = std {
            // stdlib can't parse all weird floats
            if std.is_finite() && result.is_finite() {
                // some weird edge cases stdlib parses as inf, e.g. 0e999999999999999
                assert_eq!(result, std);
            }
        }
    }};
}
macro_rules! check_lex {
    ($ty: ty, $s: expr) => {{
        let v = lexical_core::parse::<$ty>($s.as_bytes()).unwrap();
        check!($ty, $s, v);
    }};
}
macro_rules! check_f32 {
    ($($tt:tt)+) => {
        check!(f32, $($tt)+)
    }
}
macro_rules! check_f64 {
    ($($tt:tt)+) => {
        check!(f64, $($tt)+)
    }
}
macro_rules! check_f32_lex {
    ($s: expr) => {
        check_lex!(f32, $s)
    };
}
macro_rules! check_f64_lex {
    ($s: expr) => {
        check_lex!(f64, $s)
    };
}
macro_rules! check_f32_inf {
    ($s:expr) => {
        check!(f32, $s, inf)
    };
}
macro_rules! check_f32_neg_inf {
    ($s:expr) => {
        check!(f32, $s, neg_inf)
    };
}
macro_rules! check_f64_inf {
    ($s:expr) => {
        check!(f64, $s, inf)
    };
}
macro_rules! check_f64_neg_inf {
    ($s:expr) => {
        check!(f64, $s, neg_inf)
    };
}

#[test]
fn test_f64_inf() {
    check_f64_inf!("INF");
    check_f64_inf!("INFINITY");
    check_f64_inf!("infinity");
    check_f64_inf!("inf");
    check_f64_inf!("1234456789012345678901234567890e9999999999999999999999999999");
    check_f64_inf!("1.832312213213213232132132143451234453123412321321312e308");
    check_f64_inf!("2e30000000000000000");
    check_f64_inf!("2e3000");
    check_f64_inf!("1.8e308");
    check_f64_inf!("1.9e308");

    check_f64_neg_inf!("-INF");
    check_f64_neg_inf!("-INFINITY");
    check_f64_neg_inf!("-infinity");
    check_f64_neg_inf!("-inf");
    check_f64_neg_inf!("-2139879401095466344511101915470454744.9813888656856943E+272");
}

#[test]
fn test_f64_long() {
    check_f64!(
        "\
         9355950000000000000.000000000000000000000000000000000018446744073709551616000001\
         84467440737095516161844674407370955161407370955161618446744073709551616000184467\
         44073709551616600000184467440737095516161844674407370955161407370955161618446744\
         07370955161600018446744073709551616018446744073709556744516161844674407370955161\
         40737095516161844674407370955161600018446744073709551616018446744073709551611616\
         00018446744073709500184467440737095516160018446744073709551616001844674407370955\
         11681644674407370955161600018440737095516160184467440737095516161844674407370955\
         16160001844674407536910751601611616000184467440737095001844674407370955161600184\
         46744073709551616001844674407370955161618446744073709551616000184495516161844674\
         4073709551616000184467440753691075160018446744073709",
        hexf64("0x1.03ae05e8fca1cp+63")
    );
    check_f64!(
        "\
         2.225073858507202124188701479202220329072405282794390378143031338374351073192441\
         94686754406432563881851382188218502438069999947733013005649884107791928741341929\
         29720097048195199306799329096904278406473168204156592672863293363047467012331685\
         29834221527445172608358596545663192828352447877877998943107797838336991592885945\
         55213714181128458251145584319223079897504395086859412457230891738946169368372321\
         19137365897797772328669884035639025104444303545739673370658398105542045669382465\
         84137476071559811765738776267476659123871999319040063173347090030127901881752034\
         47190250028061277777916798391090578584006464715943810511489154282775041174682194\
         13395246668250343130618158782937900420539237507208336669324158000275839111885418\
         8641513168478436313080237596295773983001708984375e-308",
        hexf64("0x1.0000000000002p-1022")
    );
    check_f64_inf!(
        "\
         14384566631413902735261182076422355811832278452463312311626366537903681520913941\
         96930365828634687637948157940776599182791387527135353034738357134110310609455693\
         90082419354977279201654318268051974058035436546798544018359870131225762454556233\
         13970183299286131961255902741877200739148180625308303165331580986249841188892982\
         81371812288789537310599037529113415438738954894752124724983067241108764488346454\
         37669901867307840475112141480493722424080599312381693232622368309077056159757045\
         77939329858261626042558845291341263962822021265262533893834218067279545885255961\
         14379801269094096329805054803089299736996870951258573010877404407451953846698609\
         19821392688269207855703322826525930548119852605981316446918758669325733577952202\
         04076454986842633399219052275566166981299674128912822316855046606712779271982900\
         09824680186319750978665734576683784255802269708917361719466043175201158849097881\
         37047711185017157986905601606166617302905958843377601564443970505037755427769614\
         39282780934537928038462527159660167332226464423828921239400524413468224297215938\
         84378212558701004356924243030059517489346646577724622498919752597382095222500311\
         12418182351225107135618176937657765139002829779615620881537508915912839494571051\
         58613344862671017974971111259092725051947928708896171797587034426080161433432621\
         59998149700606597792535574457560429226974273443630323818747730771316763398572110\
         87495998192373246307688452867739265415001026982223940199342748237651323138921235\
         35835735663769155726509168665536123661873789595549835667127670933729060301889762\
         20169058025354973622211666504549316958271880975697143546564469806791358707318873\
         07570838334500409015197406832583817753126695417740666139222980134999469594150993\
         5655355652985723782153570084089560139142231.738475042362596875449154552392299548\
         94713816208169416867534067784380761312978044932336375902701297246698737092181681\
         31626587547265451210905455072402670004565947865409496052607224619378706306348749\
         91729398208026467698131898691830012167897399682179601734569071423681e-733"
    );
    check_f64_lex!(
        "\
         0.000000000000000000000000000000000000000000000000000000000000000000000000000000\
         00000000000000000000000000000000000000000000000000000000000000000000000000000000\
         00000000000000000000000000000000000000000000000000000000000000000000000000000000\
         00000000000000000000000000000000000000000000000000000000000000000000044501477170\
         14402272114819593418263951869639092703291296046852219449644444042153891033059047\
         81627017582829831782607924221374017287738918929105531441481564124348675997628212\
         65346585071045737627442980259622449029037796981144446145705102663115100318287949\
         52795966823603998647925096578034214163701381261333311989876551545144031526125381\
         32666529513060001849177663286607555958373922409899478075565940981010216121988146\
         05258742579179000071675999344145086087205681577915435923018910334964869420614052\
         18289243144579760516365090360651414037721744226256159024466852576737244643007551\
         33324500796506867194913776884780053099639677097589658441378944337966219939673169\
         36280457084866613206797017728916080020698679408551343728867675409720757232455434\
         770912461317493580281734466552734375"
    );
    check_f64_lex!(
        "\
         0.000000000000000000000000000000000000000000000000000000000000000000000000000000\
         00000000000000000000000000000000000000000000000000000000000000000000000000000000\
         00000000000000000000000000000000000000000000000000000000000000000000000000000000\
         00000000000000000000000000000000000000000000000000000000000000000000022250738585\
         07200889024586876085859887650423112240959465493524802562440009228235695178775888\
         80375915526423097809504343120858773871583572918219930202943792242235598198275012\
         42041788969571311791082261043971979604000454897391938079198936081525613113376149\
         84204327175103362739154978273159414382813627511383860409424946494228631669542910\
         50802018159266421349966065178030950759130587198464239060686371020051087232827846\
         78843631944515866135041223479014792369585208321597621066375401613736583044193603\
         71477835530668283453563400507407304013560296804637591858316312422452159926254649\
         43008368518617194224176464551371354201322170313704965832101546540680353974179060\
         22589503023501937519773030945763173210852507299305089761582519159720757232455434\
         770912461317493580281734466552734375"
    );
}

#[test]
fn test_f64_general() {
    check_f64!(1.1920928955078125e-07);
    check_f64!("-0", -0.0);
    check_f64!(
        "1.0000000000000006661338147750939242541790008544921875",
        1.0000000000000007
    );
    check_f64!(
        "1090544144181609348835077142190",
        hexf64("0x1.b8779f2474dfbp+99")
    );
    check_f64!(2.2250738585072013e-308);
    check_f64!(-92666518056446206563E3);
    check_f64!(-92666518056446206563E3);
    check_f64!(-42823146028335318693e-128);
    check_f64!(90054602635948575728E72);
    check_f64_lex!(
        "\
         1.000000000000001885589208702234638701745660206917535153946435506630705583683732\
         21972569761144603605635692374830246134201063722058e-309"
    );
    check_f64!("0e9999999999999999999999999999", 0.0);
    check_f64!(-2402844368454405395.2);
    check_f64!(2402844368454405395.2);
    check_f64!(7.0420557077594588669468784357561207962098443483187940792729600000e+59);
    check_f64!(7.0420557077594588669468784357561207962098443483187940792729600000e+59);
    check_f64!(-1.7339253062092163730578609458683877051596800000000000000000000000e+42);
    check_f64!(-2.0972622234386619214559824785284023792871122537545728000000000000e+52);
    check_f64!(-1.0001803374372191849407179462120053338028379051879898808320000000e+57);
    check_f64!(-1.8607245283054342363818436991534856973992070520151142825984000000e+58);
    check_f64!(-1.9189205311132686907264385602245237137907390376574976000000000000e+52);
    check_f64!(-2.8184483231688951563253238886553506793085187889855201280000000000e+54);
    check_f64!(-1.7664960224650106892054063261344555646357024359107788800000000000e+53);
    check_f64!(-2.1470977154320536489471030463761883783915110400000000000000000000e+45);
    check_f64!(-4.4900312744003159009338275160799498340862630046359789166919680000e+61);
    check_f64!("+1", 1.0);
    check_f64!(
        "1.797693134862315700000000000000001e308",
        1.7976931348623157e308
    );
    check_f64!("3e-324", hexf64("0x0.0000000000001p-1022"));
    check_f64!("1.00000006e+09", hexf64("0x1.dcd651ep+29"));
    check_f64!("4.9406564584124653e-324", hexf64("0x0.0000000000001p-1022"));
    check_f64!("4.9406564584124654e-324", hexf64("0x0.0000000000001p-1022"));
    check_f64!("2.2250738585072009e-308", hexf64("0x0.fffffffffffffp-1022"));
    check_f64!("2.2250738585072014e-308", hexf64("0x1.p-1022"));
    check_f64!("1.7976931348623157e308", hexf64("0x1.fffffffffffffp+1023"));
    check_f64!("1.7976931348623158e308", hexf64("0x1.fffffffffffffp+1023"));
    check_f64!(4503599627370496.5);
    check_f64!(4503599627475352.5);
    check_f64!(4503599627475353.5);
    check_f64!(2251799813685248.25);
    check_f64!(1125899906842624.125);
    check_f64!(1125899906842901.875);
    check_f64!(2251799813685803.75);
    check_f64!(4503599627370497.5);
    check_f64!(45035996.273704995);
    check_f64!(45035996.273704985);
}

#[test]
fn test_f32_inf() {
    check_f32_inf!("INF");
    check_f32_inf!("INFINITY");
    check_f32_inf!("infinity");
    check_f32_inf!("inf");
    check_f32_inf!("1234456789012345678901234567890e9999999999999999999999999999");
    check_f32_inf!("2e3000");
    check_f32_inf!("3.5028234666e38");

    check_f32_neg_inf!("-INF");
    check_f32_neg_inf!("-INFINITY");
    check_f32_neg_inf!("-infinity");
    check_f32_neg_inf!("-inf");
}

#[test]
fn test_f32_basic() {
    check_f32!(1.00000006e+09);
    check_f32!(1.4012984643e-45);
    check_f32!(1.1754942107e-38);
    check_f32!(1.1754943508e-45);
    check_f32!("-0", -0.0);
    check_f32!("1090544144181609348835077142190", hexf32("0x1.b877ap+99"));
    check_f32!(1.1754943508e-38);
    check_f32!(30219.0830078125);
    check_f32!(16252921.5);
    check_f32!(5322519.25);
    check_f32!(3900245.875);
    check_f32!(1510988.3125);
    check_f32!(782262.28125);
    check_f32!(328381.484375);
    check_f32!(156782.0703125);
    check_f32!(85003.24609375);
    check_f32!(43827.048828125);
    check_f32!(17419.6494140625);
    check_f32!(15498.36376953125);
    check_f32!(6318.580322265625);
    check_f32!(2525.2840576171875);
    check_f32!(1370.9265747070312);
    check_f32!(936.3702087402344);
    check_f32!(411.88682556152344);
    check_f32!(206.50310516357422);
    check_f32!(124.16878890991211);
    check_f32!(50.811574935913086);
    check_f32!(17.486443519592285);
    check_f32!(13.91745138168335);
    check_f32!("7.5464513301849365", hexf32("0x1.e2f90ep+2"));
    check_f32!(2.687217116355896);
    check_f32!("1.1877630352973938", hexf32("0x1.30113ep+0"));
    check_f32!(0.7622503340244293);
    check_f32!("0.30531780421733856", hexf32("0x1.38a53ap-2"));
    check_f32!("0.21791061013936996", hexf32("0x1.be47eap-3"));
    check_f32!("0.09289376810193062", hexf32("0x1.7c7e2ep-4"));
    check_f32!(0.03706067614257336);
    check_f32!(0.028068351559340954);
    check_f32!("0.012114629615098238", hexf32("0x1.8cf8e2p-7"));
    check_f32!("0.004221370676532388", hexf32("0x1.14a6dap-8"));
    check_f32!(0.002153817447833717);
    check_f32!("0.0015924838953651488", hexf32("0x1.a175cap-10"));
    check_f32!(0.0008602388261351734);
    check_f32!("0.00036393293703440577", hexf32("0x1.7d9c82p-12"));
    check_f32!(0.00013746770127909258);
    check_f32!(16407.9462890625);
    check_f32!("1.1754947011469036e-38", hexf32("0x1.000006p-126"));
    check_f32!("7.0064923216240854e-46", hexf32("0x1.p-149"));
    check_f32!(8388614.5);
    check_f32!("0e9999999999999999999999999999", 0.);
    check_f32!(
        "4.7019774032891500318749461488889827112746622270883500860350068251e-38",
        4.7019774032891500318749461488889827112746622270883500860350068251e-38
    );
    check_f32_lex!(
        "\
         3.141592653589793238462643383279502884197169399375105820974944592307816406286208\
         9986280348253421170679"
    );
    check_f32!(
        "2.3509887016445750159374730744444913556373311135441750430175034126e-38",
        2.3509887016445750159374730744444913556373311135441750430175034126e-38
    );
    check_f32!("+1", 1.);
    check_f32!("7.0060e-46", 0.);
    check_f32!("3.4028234664e38", hexf32("0x1.fffffep+127"));
    check_f32!("3.4028234665e38", hexf32("0x1.fffffep+127"));
    check_f32!("3.4028234666e38", hexf32("0x1.fffffep+127"));
    check_f32_lex!(
        "\
         0.000000000000000000000000000000000000011754943508222875079687365372222456778186\
         655567720875215087517062784172594547271728515625"
    );
    check_f32_lex!(
        "\
         0.000000000000000000000000000000000000000000001401298464324817070923729583289916\
         13128026194187651577175706828388979108268586060148663818836212158203125"
    );
    check_f32_lex!(
        "\
         0.000000000000000000000000000000000000023509885615147285834557659820715330266457\
         17985517980855365926236850006129930346077117064851336181163787841796875"
    );
    check_f32_lex!(
        "\
         0.000000000000000000000000000000000000011754942106924410754870294448492873488270\
         52428745893333857174530571588870475618904265502351336181163787841796875"
    );
}

#[test]
fn test_f64_pow10() {
    for i in -308..=308 {
        let s = format!("1e{}", i);
        let v = f64::from_str(&s).unwrap();
        assert_eq!(fast_float::parse::<f64, _>(s).unwrap(), v);
    }
}

#[test]
fn test_f32_pow10() {
    for i in -38..=38 {
        let s = format!("1e{}", i);
        let v = f32::from_str(&s).unwrap();
        assert_eq!(fast_float::parse::<f32, _>(s).unwrap(), v);
    }
}
