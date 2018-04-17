use gmp::mpz::Mpz;
use std::collections::HashSet;

macro_rules! hashset {
    ( $( $x:expr ),* ) => {
        {
            let mut set = ::std::collections::HashSet::new();
            $(
                set.insert($x);
            )*
            set
        }
    }
}

lazy_static! {
    pub static ref KNOWN_FIRST_CHAIN: HashSet<Mpz> = get_known_first_chain();
}

lazy_static! {
    pub static ref KNOWN_SECOND_CHAIN: HashSet<Mpz> = get_known_second_chain();
}

lazy_static! {
    pub static ref KNOWN_BITWIN_CHAIN: HashSet<Mpz> = get_known_bi_twin_chain();
}

fn get_known_first_chain() -> HashSet<Mpz> {
    hashset![
        Mpz::from_str_radix("18088387217903330459", 10).unwrap(),
        Mpz::from_str_radix("4611686018427395339", 10).unwrap(),
        Mpz::from_str_radix("4611686018430476039", 10).unwrap(),
        Mpz::from_str_radix("14961027376987860299", 10).unwrap(),
        Mpz::from_str_radix("18387163427137165079", 10).unwrap(),
        Mpz::from_str_radix("201753609400367309", 10).unwrap(),
        Mpz::from_str_radix("33376463607021642560387296949", 10).unwrap(),
        Mpz::from_str_radix("36857073504463708989820858829", 10).unwrap(),
        Mpz::from_str_radix("65850116006148786152439799199", 10).unwrap(),
        Mpz::from_str_radix("77162903328470141405988589789674379619", 10).unwrap(),
        Mpz::from_str_radix("134382569868724676622974714529481507019", 10).unwrap(),
        Mpz::from_str_radix("170141183460469231731687303717167733089", 10).unwrap(),
        Mpz::from_str_radix("595374401003766034096130243798882341754528442149", 10).unwrap(),
        Mpz::from_str_radix("365375409332725729550921208179070754913983243889", 10).unwrap(),
        Mpz::from_str_radix("1332079220031954145589251158141208020515543604929", 10).unwrap(),
        Mpz::from_str_radix("1315910738258594946877020432332324419730043990204002549999", 10).unwrap(),
        Mpz::from_str_radix("5286099634025858841161357417667683784807437672358608696939", 10).unwrap(),
        Mpz::from_str_radix("26492105385435541326705069945527933737713984117118578345330797608979", 10).unwrap(),
        Mpz::from_str_radix("11757970121934327541360733702827942876206847201076324344452911002529", 10).unwrap(),
        Mpz::from_str_radix("28948022309329048855892746252171976963317496166410141009864396001978284493479", 10).unwrap(),
        Mpz::from_str_radix("48485404941743174450917141906484355462490237658145525006839828940029456351669", 10).unwrap(),
        Mpz::from_str_radix("86234486510746340125137830122141702027347200257363118510992330588951381407989", 10).unwrap(),
        Mpz::from_str_radix("108693781201411804277652435771198207892449933958867996774894347899368414395719", 10).unwrap(),
        Mpz::from_str_radix("28948022309329048855892746252171976963317496166410141009864396001978300618419", 10).unwrap(),
        Mpz::from_str_radix("113910913923300788319699387848674650656041243163866388656000063249848353322899", 10).unwrap(),
        Mpz::from_str_radix("32513809984092380819192652088162513959582762326769189004986614263354006979172021253243731928068713186557336122560969", 10).unwrap(),
        Mpz::from_str_radix("12307039909855129437896451704872238558838052289096716166011015803484525435222231681762227344644768174830461668844219", 10).unwrap(),
        Mpz::from_str_radix("19701003098197239606139520050071806902539869635232723333974146702122860885748605305707133127442457820403314808603969", 10).unwrap(),
        Mpz::from_str_radix("4257288688009878954451695431375817659515008633535285807045060622927544588698238829010900595221975499571076126680139", 10).unwrap(),
        Mpz::from_str_radix("6649653491081530622612538954981514863756002949295969609441086304155754360299515994556752300718232934858988121957955252900133322489991133429597175723068529", 10).unwrap(),
        Mpz::from_str_radix("6703903964971298549787012499102923063739682910296196688861780721860882015036773488400937149083451713845015929093243025426876941405973284973216824935693999", 10).unwrap(),
        Mpz::from_str_radix("2114808150280136020594982449559271743100598184592768859100325004053601242007840417934409418223841139254332921492561624155776746447081613720745982050813711487912144580903765643976083914728571299", 10).unwrap(),
        Mpz::from_str_radix("1186387563771966323021035874369368132807321218190992295127185928992437744187507398449740416674504973717870497985077219660782409185450455176824057976730967985420790374164677914043888492047407062040237294767414832210954126767988140179", 10).unwrap(),
        Mpz::from_str_radix("203274743466338590506856676084692781871543449315690960832874378776482018697599795319503031064142081441580957059007386501168798841524204630787816838492320705090398088227343584066754152886572610125242424604166426067474385045647594683321692242457119953411978836810279307559", 10).unwrap(),
        Mpz::from_str_radix("153739637779647327330155094463476939112913405723627932550795546376536722298275674187199768137486929460478138431076223176750734095693166283451594721829574797878338183845296809008576378039501400850628591798770214582527154641716248943964626446190042367043984306973709604255015629102866732543697075866901827761489", 10).unwrap(),
        Mpz::from_str_radix("37313426856874901938110133384605074194791927500210707276948918975046371522830901596065044944558427864187196889881993164303255749681644627614963632713725183364319410825898054225147061624559894980555489070322738683900143562848200257354774040241218537613789091499134051387344396560066242901217378861764936185029", 10).unwrap(),
        Mpz::from_str_radix("89884656743115795386465259539451236680898848947115328636715040578866337902750481566354238661203768010560056939935696678829394884407208311246423715319737062188883946712432742638151109800623047059726541476042502884419075341171231440736956555270413618581675255342293149119973622969239858152417678164815053566739", 10).unwrap()
    ]
}

fn get_known_second_chain() -> HashSet<Mpz> {
    hashset![
        Mpz::from_str_radix("8200568588273131201", 10).unwrap(),
        Mpz::from_str_radix("7119585911130398911", 10).unwrap(),
        Mpz::from_str_radix("10778533281280055611", 10).unwrap(),
        Mpz::from_str_radix("3623684585367099991", 10).unwrap(),
        Mpz::from_str_radix("4611686018428091431", 10).unwrap(),
        Mpz::from_str_radix("861715730411462341", 10).unwrap(),
        Mpz::from_str_radix("1674986350879671961", 10).unwrap(),
        Mpz::from_str_radix("67900120189714574549279004031", 10).unwrap(),
        Mpz::from_str_radix("10101837493672093280040555361", 10).unwrap(),
        Mpz::from_str_radix("72715672387515028471963509301", 10).unwrap(),
        Mpz::from_str_radix("39614081257132168796774190841", 10).unwrap(),
        Mpz::from_str_radix("10649588144161723271997444271", 10).unwrap(),
        Mpz::from_str_radix("41918868722453945727647602591", 10).unwrap(),
        Mpz::from_str_radix("467163544356581123923693233169765411", 10).unwrap(),
        Mpz::from_str_radix("78611250347504386688211060697879905631", 10).unwrap(),
        Mpz::from_str_radix("39313361333713821365071953370171601071", 10).unwrap(),
        Mpz::from_str_radix("296998440399300067472060767080169347567163272451", 10).unwrap(),
        Mpz::from_str_radix("655996338119965871537721549051574508184037198531", 10).unwrap(),
        Mpz::from_str_radix("127047554482172410847857928619907738690147109078632566751", 10).unwrap(),
        Mpz::from_str_radix("22778114142211962207960213784135598105906811222236549595128806545751", 10).unwrap(),
        Mpz::from_str_radix("11477799997239866295941478166154077568792584042185282664607488558757512965001", 10).unwrap(),
        Mpz::from_str_radix("9243036475383693019184249558391683043620596247492679410852220359983850670306393322662256232486961325798592165759821", 10).unwrap(),
        Mpz::from_str_radix("5431904924836803914172058287993712865714788953873096810910041945833005800417312386015285236741498469319199765740271", 10).unwrap(),
        Mpz::from_str_radix("1306496017242772172743264447276527112501750362327928440722559572994619766769639623457450192156124916040769111470258706639743561430669385023580621264713531", 10).unwrap(),
        Mpz::from_str_radix("2281220308811097609320585802850145662446614253624279965289596258949637583604338693252956405658685699889321154786797203655344352360687718999126330659861107094125997337180132475041437098767579101", 10).unwrap(),
        Mpz::from_str_radix("252697908391054572799117421345469774591354691874560524506502892530508543928591733438162392467541591869402667175721583754581423928309075337803934872666481782932999857025695043830042417141857167159879947804010079918402116994974115121", 10).unwrap(),
        Mpz::from_str_radix("308208773974777316834938321954665476210798675683182889883680370374451855717140297440376220160579261376115669942382819852769344056844518999959371899569689653644159219223887105646071048248480190862304309611157578985702413599112798317769143641045928117423782520394794222771", 10).unwrap()
    ]
}

fn get_known_bi_twin_chain() -> HashSet<Mpz> {
    hashset![
        Mpz::from_str_radix("6400306986398717280", 10).unwrap(),
        Mpz::from_str_radix("3355006646840301600", 10).unwrap(),
        Mpz::from_str_radix("15702824270839018740", 10).unwrap(),
        Mpz::from_str_radix("14834106023628140130", 10).unwrap(),
        Mpz::from_str_radix("9223372036882100790", 10).unwrap(),
        Mpz::from_str_radix("40407349974688373120161404540", 10).unwrap(),
        Mpz::from_str_radix("31304362342553789833295320950", 10).unwrap(),
        Mpz::from_str_radix("39305625173752193344105530353982143730", 10).unwrap(),
        Mpz::from_str_radix("392393888434791734846979273737983457731877192100", 10).unwrap()
    ]
}

