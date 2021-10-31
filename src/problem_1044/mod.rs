//
// Problem 1044. Longest Duplicate Substring (Hard)
// https://leetcode.com/problems/longest-duplicate-substring/
//
// 168ms (100%)/5.9MB
// Space complexity: O(n)
// Runtime complexity: O(n * log n)
//
// test problem_1044::tests::single_long     ... bench:  26,565,777 ns/iter (+/- 479,307)
// test problem_1044::tests::string_1k       ... bench:      23,895 ns/iter (+/- 258)
// test problem_1044::tests::string_1k_same  ... bench:      19,971 ns/iter (+/- 450)
//

struct Solution {}

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        fn rk(s: &[u8], len: usize) -> Option<usize> {
            if len == 0 {
                return Some(0);
            }
            let limit = s.len() - len + 1;
            let mut hm = std::collections::HashMap::<_, Vec<_>>::with_capacity(limit);
            for i in 0..limit {
                let digest = ((s[i] as u32) * 65536)
                    ^ ((s[i + len - 1] as u32) * 256)
                    ^ (s[i + (len - 1) / 2] as u32);

                let indexes = hm.entry(digest).or_insert_with(|| Vec::with_capacity(4));
                for idx in indexes.iter() {
                    if (0..len).all(|j| s[i + j] == s[idx + j]) {
                        return Some(*idx);
                    }
                }
                indexes.push(i);
            }
            None
        }
        let s = s.as_bytes();
        let (mut l, mut r, mut idx) = (0, s.len() - 1, 0);
        while l <= r {
            let m = l + (r - l) / 2;
            if let Some(i) = rk(s, m) {
                idx = i;
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        s[idx..idx + r].iter().map(|&b| b as char).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const LONG_TEST: &str = "\
    uirqkccmgnpbqwfkmplmfovxkjtgxbhwzqdlydunpuzxgvodnaanxpdiuhbitfllita\
    ufvcmtneqmvibimvxmzvysignmsfykppdktpfhhcayfcwcrtjsgnyvzytevlqfabpjw\
    xvjtfksztmdtrzgagdjtpeozzxppnkdbjayfbxqrqaefkcdbuvteobztucgdbcmlgnx\
    ldcwtzkvhwqxbsqlbsrbvesemhnbswssqvbiketdewfauvtzmyzrrqslzagjcyuznkp\
    kgpkkinldxcjtuoumcbcttabfuzjbtoqjqbpnsemojbtctvdmobnuutcsfmhkrbwkmp\
    dcdncqizgtlqekvqophqxewkpxpkrgjivrtarxtwbbfhfikbrdwohppyiyyztopksdm\
    xvqiaafswyjfnlwntldwniypzaxrscyqfrlqehqgzaxxazfwtwjajsrmwuerdqklhwo\
    qxptcvqoqbjwfqrewirtcbskmkaqgtcnxnsqmwgwjxruhyyjtbuivvepnxiwjmtlrve\
    xjzevctflajibxkvmbzdfwoqobmhstgpshtxitwttpkdvfmfwtwsazfgzwbtmqrowrc\
    esyyeuwunodesrzbmjbxnchaqptfgqlknuarhgnsbwnucdhckpbwhtwhejivrmuccbw\
    nyenyvonquscneswngwbkjysxvdwbzymwxcrnexrxhmuwvycmsiazmqavgmyurbcmvd\
    jplloidbzacunerwobvaxsromiiwzqxnrsjpyoacfxcmmogmokhpmhxzkdzmpjcrgae\
    ihdhczrmxmfurjatuwxriiwtfojwvkkybcdmwayhnzrnqrynwtrvmtgtrxndlbtlhyz\
    fjtbtvqujjuwpibuonuwjdfvnhdqqzlmwwheztjkrrzrroogovapywxkjsccjnseanh\
    xijybintgbjwlkmdzuoeclfqatffgjvcbujovunnauprhoocxzghzvsmuyhslnwqlcs\
    dprwutyqggxfvczqiqeatglhubmllcvcqxrvcojllcoufzfxlunpcpfmypcjbnhtnhg\
    yxriupmpvioqnkibldpnjxctyycwdxloucneypbxwiehqmzkvmwymuurykodpvylicn\
    okwwencbbvzviljapeabhjagrzrmvgebwyrmctksubnhawahjclnpmavhsirvewqbuy\
    mdwxcsxtcwehreejtwoagewkmyhwzvyurouyffwhnsyhpiiozetsoveeqndctcxpght\
    xkpaolqjfaiwdtfjxkvsvamtmhyzuqttiaqhbkzxdjoqxnwpqnykcfaoteretoektil\
    eadlqizzlfgcwyhtgyjxdznvaohkgukmhjspqxgdygnulmmrwpwpkvlesvkqtuihzom\
    yxedvalptrineagfdeotlvapabqrebqszkdzekxvttytwfzlsmzyfihcrvsirociunh\
    uplyaukcoiyogdbtooijyfazmfpqwzfgdqzzcggnytspjihxeekoiafknmdvmeppdgs\
    byeqlpxhjkibcrhknfkznuqanixzckkcufqkvfbcmvrmboqkfablshfdcqcuyjqybsg\
    zpdnjcpzekrydnoxjpwyttblbcadjifmrpexjvunecxnkgeusxwqxujanhdfjuaijng\
    ssxxgldxckiucpclhmzcwdhpvdothkdkwnopugrxffhqwwxtxvbralbzpzsgtkdplyd\
    ydrlckyzzsxypywnavgnkxdbgptkeagvdhigxbnksioopfpplizvztoqrpwkyzazqvw\
    xirablkyvyxfvemgtyrwubpdsuwzpahsnjepppwadoqeawrzhgrvuxexrqikkfpcihl\
    ypsnuhqnwzjfkxbrszsgzdmoomlojemujdwhpdinhmwaagbgovhziwrvpwszsvobdxb\
    ccxonqilhggfvnejiixcitcpisomwchguxhchizeyxtshuyqmjlhnrvhbbqgultmndq\
    bojwqwfzqzlwvqazixjrivjcignlfhqfrmqhxpcarwbwwkepecdezudavkfkfdefzrr\
    ovazszvexdjtmqdaazzqhydakrvhihnmmetpdqalkhlkaavvpolsfzehmhpslrbscyw\
    gybctiunwxbqkojxjmyixqfoxwkpowukicsdgkonbqqqgawqgcweywlyntjonqfrzzl\
    nefktbfrutbeqsjktlvvinrpcnorybwuocjothvohrnjeatbkuluguhazkhfruhobjt\
    bnkfqqawftrrrmckzopldnhmwdrzbxdfbvbnmpxnymipabizlmjfmufzsjvazutsvvx\
    dytuiwgmredaceyoyrjtxaikgtxnirdfwaffydnufzjvfsotsnvtpuueygzicyvvdnq\
    mqsgbdbutwydtadejpjjybguekluohpmmazslvyrlqjtellkzbfvethnsoayrjhmtjy\
    wkczieszgzrncxavcdpovmqwofeaqeksfonmdjfcdqwzecgjaciriymkklabtmklirc\
    uomrdpeqtgkwlyqbzxditppvyhdnajwfcuedtxlmdkhhaggjolunfpojakvgkamiwcc\
    ghznakxwvqgpfctbeadvcuwijlaitmubgqkbazlrutvlsahzhzxhydedramruudkqfq\
    eycvyrclkuomsyyqarlwpmoqvtqlflscjrpctscizelgffwkhlftpihqgnuvkdfqkaf\
    cufpkhosbtdeycfhwbhjiwqbwpwicwywafqvazkjiibtohdbaqhvfdntufauxcsjkga\
    obbyxmgfoqxggnhwpifegjehpgnnoshkjbwyleevrnzfzxrhlmlittjcxjjaboyuepu\
    fctqfyiaoalxcindpluunrxvnegslpigqofsqozkxigbgokkcpougplkqdpermnnpge\
    lgxogrbiufvzphzhnrswkkcuscnuxrfenhjhwjbzmydfnughyifenxtlqpeexmsuzuz\
    gbfebbdyrirzvgrvdmlagbbycqqwsxzeqyjokccevcebniyxzznxqkqrvizkncfxbmp\
    absigucewylsghvthkauqddarvfuoxpxpowtflhxxspbsncyekwtiywmyakrnxqphdx\
    kwwlmvdilqsjkraemegbslpkzrfggebukefcqwsliqjwxkpnjzmqchicyqnvacismzg\
    qncurzuqpwwpjnpsgtgxszraeavhopzcrsbvemrwxjvctjwjrjjkjlesmypjxnoidpr\
    odufuskpuphcaudfqhyyhlczztqqrwlqvzsgmqnwizrgnuzaxqtekjdznxxxbhubszw\
    jxvynpyifllqjvxebxyfqvwuojitusnbutylahvzinydvqvlhezxkbemtrklswlkmip\
    oidiiwilqrdamjreetphgexhrhflyevdmxetsctrhorjesmuzdgtdirwqftgvomtbjv\
    wymubuajfvevecruuppxhyotbdfhzvtiwewaiypqublanrwotmehnhdojgmlslifsge\
    tpnaetavkxujboxakttvgrkadnwzhslaryrvbedtrahddsnpuapxruqvvctrjtsskzi\
    knuonrglqcmtpwtdaakhlkxneqxkchrfkpwznektfadgwetfhwnmzupkerdouulifjm\
    sxvamgeumgigdldtpeklgmlppjizsocvljmuckqtotdbhhavclarnifecmqmjomotpa\
    thuyisjpbnkzgyslvnsxctuzwwdyxikjvfyizqnvqovtenxsxybateirdeunrtknnhi\
    wfypfscisinkwjnassykklybrfdsrapuebvjpshezxctfydulllekuikdbndjnessrr\
    knotidmsdfertiertauzfspljdvnuybmjideoutgimgcecyqxgkfglgfkkytgqoznri\
    zealqpzfzrahsxcyngkiynhggpxietbgynflyufwchqdlcmrvfdhqkbmyctsikvnugs\
    wgdmvfacbvnzdptroagieqqwaxkojcipzwvdzfkqcscickeabjaoutesvlpopplthuf\
    rakrbdhsbuasrqvrfydalelazvbcvcqsacepoqouqqopeybmcohwilxbeqmkfxihmyl\
    uexnihndcgpwjllbingqkjtdkjhffxtkastzkjnhmszfqcnypbkgkljjicftjnhvqec\
    iglviiwymqtgxdzmgsbhhwkorahiameuzsurojpiwueddmhaagkghaufmywsdgvvowd\
    eqmrchnkrmipzemfkglslblkjjdjpbensjtbzxvdnvrhupssafrjctzrkorqycmfjmu\
    jvrqoikhdkligdkwxyzhsrijskewjmjyzhxvbmillrwlbdqstesbbehnoncxykhfhtd\
    gvbslbdmvlabywwwfsnomjzwuyeynzcmyjbaszxcpeozuubvhbufxaojzynajrzaylq\
    vfmrxnxfdsthsnwwtwsavuzvmyerwfnykcgppjoioqhpwynzpidywlxdboxkoujatbc\
    yqmutrfyrsjaxeoroziuhprntwnjzknpkgztfyfongmuwhcogbqabvvsvptkdmlvivp\
    hbrznsjrpprnzfdogrtgyqljoarmioogfiyvqchhmypiqwnltudacqrqiqmjbgviwjo\
    duuhalovvhboumlgfbxvbvfgqikgzyicoaitvtbpsficbcgutbtuxbfqwrdfjwwgoxj\
    jhhtxrcuhknwqrjwfoslypnjonlzglexsfxpqedugjtllqiitflwmqfkfowkhitasbv\
    howvwrbnxfumkjcbwyzxgfftccptxxeomhnrzatzepkmnyogiepvguvfvfyfmpkjnuu\
    fikxwhcvvrjmignitnvjjspxvgjxyqwfdxpszcmzihirrvcxyllhbdrhhoiratcmmgf\
    zejwushbwdziqdhqfsnojgqtkpquiuxmnjyxzxzzkyexkbrgskbylbogbyrdnafioxg\
    hllkusvdkgcygdflxiqumezbfcncmodemkerbsxlekkcsqjmkrureosjywrwnfzaory\
    ajlanfouvtuofpymbvjibmdanezzlpgvvokvtcchegfdskyaihgurfjrqaeurlsckei\
    eqbiohefgqxnvwxlhodfxbukafaimjygjdodftdeytymwbemorvrckerwfjdqhgxptl\
    snhbjybjuerafqhpedrfcgepgobsxbkqttqpdxiymzkbiiyrncvnzxjcksblzvidstn\
    ntyssrhwlvizpibjrfvqfacvylcsmcafohldmwojxrtqecsvcgqplpooyutkvhurdfs\
    meagyrwckqupdvjxczshakmmbtkjwspbtzjqjzpndtegduflwriktdetumspbbitfwd\
    zfwwoyqlqnyqtsxdcdwrofvjfvoojuiauofwsmjkuukcqdohfczyrcyfmbotajdxypb\
    fgtzqsvaausdrurxnbpjdiexvpguifiptbrsrspkdrutbnthvtxbaiglpacaqacrwpb\
    xlmizijfseucvrkhofrnglzdzpksbiestjehvqoqfcefdqsbxkyylszukmmlvsqjzqp\
    nwmtwesfdccdxbndcquaxnuwbnvbpsnwnejmtrxfeglbxrpxfxgedupdxlhbdgzxyxj\
    cfeypautwvvswxuugjeeiehvgfnwjrxxkvmqbnshxbskdlxestjqiyphnizrshmszuj\
    opvsjrjambyclvlugylfikdognixfkloabzusfcyhiolmhyshbvoeqdswtusiwapcrb\
    znzgldwaamomvkwkuewpfllmomuhzvwxbickdymcrpezmjnxgtvvmywsnvpbtdzgmau\
    atblqfgwwywftylbvwqmadzvvjtvifqdbupcfmwmaiainpuhsypwmrsgcxyacuiccbw\
    xxypwjceykxxbbqixunblwdzvbohczwayyvyfenfflauuxxvarbruvpkwycuwghxzde\
    izuwqehgxshzhcbmurkvhkeszqlijcvmgyavhvgtzsdgiptmojswnrtjyehfystwugv\
    lyicggbtscbztahoowxkpprifxixsrdfyohtzwyncjgjpeslukofxtxhlctsixfvpzv\
    lmwoqmhxdjeuodkoshrlwwfnobdkaemiviuyyqedpfiqacitttrjkacklqhpnxbrbde\
    tbukeqzcgvmhrmqeudbrrryzddugmyapmqqepgufhybsobaidjrsqzgamyvoeiqsofj\
    sugrqhserhqzxpzdnncntosgwjdqclftkvkpoyiconsslpmdxxstapfyxdjrgicszlt\
    ckddxonfwmjdzsdpqoreboirfjkfkenzlktoxakfsaapvqornemrdmgbltzrjnuwnom\
    skunnemmdwtdideytcxcmqnvodwkquqpjqtjibfgvcjsjyvpncuacbhjsplgobpnufo\
    taglmfazhxgkumffkzqszkbigwurmggrwbnmmdpdgsxtxivucijynfbxijwairmyfjj\
    cuglnzrgkzonltxnpfujpvyevupcnrawumhqhapeiyouniqcyatmgrzsyaoxejsxaee\
    rucumnahrfukyxpcznvnnxlnvctnxupvrtzypummothmbyvrnkghdqqdpfqxfxgdtnq\
    yjbmdpzjmmmdgttuldpgjasumifjaluasojihpmcgmlvztefjfpgxmyliwfztkfmmev\
    byzsdxiixpnqbqspuanabznxtjzekgmkzlzjoycoexfjaoqapkknptgplsldzwfazuv\
    dxqahaqijittbgqdrnhxgpokfezbawsiwumafehzppautuogvsahkfamrdeepovlvmp\
    rmzzrkpwmweijthqmgbydwonpiaydrvpcykwbvxhmcufmonmvyecsyozzkecjiudhqh\
    uwrqvttsfnfxltftyaezjohquwyyysouasydxebixsplgwcljsljradusoccdvjginc\
    suydrqxabsfglcfefwzvfqzywpeiucgtrcweykgiwngbcasuwuwgvdjmwbabsoxrqro\
    kxyrykyjuckqiwwaisomotluenwcbqsipjveegebrqxqzugrtcqnsagrdbqmmzffmgk\
    ilaizklvmtdgbhrsvvbuvpbcrcmczuaqfnsoxvlnfpawhjdyolysoawziztgjuwtlur\
    ibermtzlqddnaqjwnhgmhvecwbyqslajxagtjadhjvhvgdovtmtqjrssrhblznkewgt\
    qrfrvholdtlsbntjrylqlphpsmmgupweeecjohjubaofjsbzcsriotfljcmflbbasjm\
    eutrnghbldmxvowgamoejejouvpoqqqahtcluznvwpjuvpfhongubixsmlyxxmpaanu\
    rokugbskjdqrkrhbkaktbkvfvoxuzftzbqkpxuvavzqyhawteweguqzyfdnznbfuuwi\
    fvxrtbahudcyjvlgroehzbgshqrthnpczyqxwlgmopubhzsbscxqsecxrexyuwyvmut\
    kzvijbolnrvpnljhvxvnzxhdbhktbfakpvchjvjxgljfvtinuuhwvfydfnaghdrsalj\
    mfghjrappzqvrokwljycarotidjiolbvailscqkudeenywudzqkjdyivecnzhfmulod\
    rqwyqrmzzuhjlktksceoelrragvlnbilflkbjydotduabakfejjamcdffknxcnnudlk\
    msjboessgqdtplhtlowirwkrvessnctcysoalwjvbzxpyxqszbhljvthaphrvrrrxeb\
    dgnewwdrumitninqihlhbrihovfcylnxgskdaxmpaklkjvdjtxolchkdmpeijkywwby\
    zdworwvxogmrpypngqawlnebybzjflbqotjvwtomgerzklfmofsqgrfjcxfupbmxhit\
    crvoozphxrgchchrckigpashhtwpxedjhbntobvpizbykcylfolblbnwrechfpjhxrs\
    fidhhxliraxwxfqfzocgehglcspmdhyngdqdoazytgokzonpagaaxwvflwuyibbapmo\
    jevlxbbpzfmzslqlyjjduwlztvfxblyqcnsfaoexmddugjhtrqostouzcccvevqircu\
    snzhkhkmsgqwzbjwaipqrnkwiorqtvzoigkbpysbvkzvivuxwtjhucgrcsevqtttddl\
    uukitfrcdxmbshxmssmdeliazkvqmgsgqayyqfuhlmwbypwqwamaqrbiuwobczbrijw\
    xxvnrkaywpqljfsinzsuczmlzrycvebgmlupoqaytovvncgwakgtvmokcawksniecsy\
    wmbmgxqukuowbujxndgcdvhhccbxdfdtupfchckinhqvqmyrgzhnkfnpghwioowhaxv\
    ljhytwmaxojaflalojflawefygwqqxsytutujwtbhibawewagxkybusekmtxrpaulrn\
    reqwndqifsggbnorcqbukdmnsdflxecjjebzppljsjnnnpbnuppmpxrflssthotalfx\
    woaubwaffzplaoaprzmyprfgdjzceihjripqmflplgooqxmmpzjoafjnttsaqgpuamb\
    xspuivihxeqbypqhsmrbjtkaektxwitfbhmbqabahriadwmmnkcahnyvpcsvmteisjo\
    pmsyfnpdvlegzhuqwcaqzxwwzzdwegutxlgbgcastyimovrmupxlxtxqglldrghginu\
    abfkgdmpfcvbuntalmljhblvwdvlcykzdnrrffbaiffkxyzlgbzvzvlswqcsqybrixl\
    mrupesrnhgemquzrhmhtpexnzknawokoaasobybyfsbrpxfraywfaumvynmjafzimwh\
    xgxgcjrlryiokemhgpaxstgggybjbzlpoblyvmwjsudirqwuboqixvohypyrgwypiww\
    fllrqopfosltidbzwvigkcdzvuuxjecuygwdiwiuwieaojwfzmtxrttybqilrgxlimb\
    frwlezzfvygdwoppadtznshehxhmaiputfxeelkjrmbojtnqnhloqeeanulpyfxrxgn\
    zkwsmtifvqkmvskvycgasppakgsugkupvtouwtrclngwqpmrswrhercdpdaoitcoard\
    zwbvftuttvjvncxqgwnufruxptxwjsiwrdvzauklioyakxcokrgpfvhcguzgzckwvhu\
    mbegecwgprpbkszoczrpqfkgsfepvbblsuwvheyggosicwzmimdclpdcfkdldzqnibr\
    lpehdbpykhdobeuehbixedbiyhghoxaqiyrwyeugqyqndmlpulxwpavavlnnirolatv\
    rpztrrlqssvqyvycqcvouaqzcjiqknjgmafjuovadsnauhjninhyccvihkldsufjisz\
    bhqbiwbqgyxeqtoxmuxtitzljiwzmieymdxahqwlmnxcnehtyrkjqirruuszaloqbzx\
    gvrvzndhhmfptuwhssclafxzgkhclvqxjeoiprniewtqobtacnffsaxvmfyynrwnrdr\
    vjbnhqaugbkwlordirucuppedcgqyplfftfpzotdtzozglvlytzobcifzzlijlqmpyl\
    zajgjmywsvdmehbtlibbdbkhcjvanmnswkasbiiinunmkmwanvoaqodteegrmlnyuee\
    rxnthouexswkjpkrfpmzblbycrfculeojmmcfdqurnxhwqnmvjbmvqqjgobubsbflbk\
    zeewsbxvgbhgbyxiqrbecylmbgotwtxebxvppmwbzmegsjjatygluakvfhfkjvtrosz\
    bbuvmyfronzazuepsmtudmvocubupodvtbokchcycfywfalgklgqgurypfnkimoixch\
    bqnfsybujgmatkqntmtsvjpkpzuskcfxejrcjoxhrguddnyeizooaaidalerhweqgol\
    uqxzexaailanyrsfnypccimkekgvoahfcynhvfdowxjywkzwwelkspispuhtoxyxacy\
    gtbgynvdeiwhgdmacbnzmqtkvxnvzokepdjbqadpkriiucdvdjwyclcikrypvjaysex\
    teyftxrlrthrxgwqgqgayxfrfzzuhjhpmuewkubcitrlnlxrsujmlkxadcizjjrgjhy\
    ljuglvkchguieohohtdulxdgvhtoxbvnuogmibtxdsakiomdnylyjuixixsvtuirmlj\
    rktgsbnajpzoneunsmqjvxjhzcslmqdyvfiopvhqeetegdbhxpveymraeshykoaljxx\
    iwjkrwkfvsppvmowfyfmrtphrryimsflivaanyeyokyaraocejzgzxgidmbsvombwqs\
    fktzrdikylpojukcwsjumalvvlmcokglqxkfewayboxmdwceygdjpfvbgindutpydlu\
    wwmdupathgosoxnhotozldalbmtkbhwahyajrsgvcuxfvpaqqecowhywrdxmjzkasfj\
    qvbumnahuplcjdhxkunakiavmrazweofqftodajueujgeeaylwiycfsycvinwhbtofp\
    dmksjwafyqftbocjhmzmvljsnrseevsuvirkunfumzdlgcuwkymwczpprpgxvekqtxg\
    dibsrwiqvguyikunecbcmfruwajkrloccwphthvcljrnoksgngvaegkrpscazqlrpjl\
    ugzzhjrifaucfwhmmmxpersfjvxbhqcuxqwunoweuxfvolmtuflwfmejuywvbukuzlp\
    mmonmkuwyyxbthcruxjplzpzhvwrozrrjwbyfihglrvpzbsueaotfrhluybzhzftqlu\
    ynwvovjidxiguchsyrhvidbscdhmyuityteugdfheewffmkthamxcviitrpyjsiqwmr\
    uhbkxqejizxuzpzsvmvrxoizkhqzzddffbptfsscgziqtnmdlavufcbftkqkiiixjfb\
    vfoxjulfaqiwqwoceigmzhzhmywwucxwtlbsgmvsqzrkaielnkecuznymtoowtyxang\
    znhvhgvjtrynbqcdirgeurzllbkawffzfwdrmpacouolnhvtnijziicbecojcskswol\
    crlciulxodqetdxzagstitmknechbzvrqrgmdeewybbxjpgwbwzzeaqhdxtfopmshev\
    gkfgehknkefxwqaukkwnfvtccoaxgrgagupepypnzbductjvwpqhmlczhsjemnzpekv\
    weoaeyhilnkfnwinryjjrgmfdjxobhqammpvwnifbzwidkkrjxjeelejmjzfluyxhdo\
    zzoikylwdzpijurxmwvrxevbhckhbpkcbpniuffhgxzepvftlehjaugjvopiwooyqyj\
    zmrutxlublxasaehftjqwpqpampjxlogjrpxgglmxyrfchbxvriuinsmuoevwmfapxp\
    skqrofofhwrfwbycxgvdrklriayrkxuyvfdblfyosdeoysmwnutitsyisawayvlrhxm\
    yzqnwctvcjoljaziuuebkktkqlujsdvmtfxjevdhqctwqtbdrokbnieefsrecyulznj\
    bqxoajnqewosaatnwxqrezftlpmyfyasyxojhdsijrbzpjggsrkmrpobaxmzbccpvef\
    dgxmkvstlodakkzsmitdcxditkclmoinjvwlrjepdepuhohhesuzehjisehnufihzdm\
    ksigbvqjhzasxrtysdweiaghspogclgkckkpivpctxwtoipbxbpmtnxsporlfpxunrh\
    lzofnidghpvauhlccjulvujwovdfqskwbunimyvkzbalocwjqcmnoaamsxrbodkmgpk\
    oxpwmqjlfsskesmnvjcqdbtdvlwqofwmsvufryeddmxioxldlabammjlwlicdfysufg\
    lfyirjaexognrtckvhxcbmbbuxruxzylklgxowstylvbdqxwnxgdutqzjxszrwzbnye\
    rmjqhrgdrbyesgoirecoswohkzchnovkvgvdrqheaahmpbadvbuyxcasdxwexsmscfa\
    pbookhedlpeegzkjuxjpjpbrpuqzactilgujdrfrxvzxphgkvrjiqiipvbgpskxyulu\
    kavvfmjblglhyucfpbwdkegxsjjhwnyxdxkvbplksfzqtrttcvzjdpeyzdjnncdblvk\
    xqrjkzhnwfhsyxtbvigvrxlxmsngigseksuctxrfpvrvamhqttmwuqkwqbifwqbcghl\
    ewuatkftuasidvclzwrbwexcgxmchgbodhxvswvdlbymvytmclgabgpnbcdkawrjqhg\
    vweyuxhjdinuzakvhhqjlyiueivpiorbhzjrphzqbtgtelqpfljwdifhthnpdqeswun\
    pzzifvfzjfahhijejrohodvforlqrhzemvkrdmqlevlfjkcgcwcwknfbvdffargdzuz\
    rwejomsfvgovovfzemvtakrwnxweqcqpoljhccpfwrupbseugipbzkcfrukwbmqrjcp\
    jtnayvitpotykaapexjfrnrmhebypdruxuuqmelojadhbaztuhmukyuedhuyweojqny\
    wmhhssbacapuqcsbmeafcfiffmfqctestvnorhjscvumlspjunfiznbqxqdllwpoyln\
    biifcgscecgzkjmvivrigbyyhmkxjjlflwbhubtfsqkxzwghfbndbkhlbgwkqcnqcja\
    lcdphyerdgtlhgfzhjgowiqrxhtmuyyxrcxlahnbguwvnxgtvstlovpnfzrzbyqwbyj\
    wtlpebzvvgolggyxqnqtuboqpurrqfwfqsxkcuqcaifeezsdqwdlgwmyhvqqkrzokre\
    hfwdoeskgdloywfzjpusfxfraciuyyhmfunepeghgxuqtxhqseqnljjxvlipcerbobs\
    sggebhwxxltgiyjfmfafzksdylbezxvdpkebhipxyndzbyscpfovbydwbqobylccawo\
    qfbtlulmoqcfnprlkxgmvidhdtznkfnjwwramsaowpzjorskjhkewtwvhfelvmfnffw\
    uyjtfsfmrxbyrtchqgjathwtqomoiafacczseirxnnneroukshpfvhdcwwsxkowaokw\
    fwijcfqxrzxpurvaubnsinuaqmzgtpjgirrgpqwxbjwtxrzmlagwyqlhtzsjbzsampf\
    jwidlgkkprtezrnmvlhiivvlmqytufmkbmknnavnuwkubnpbreivfcplvubdcqffwhq\
    nscmbfstrxeltkbicbhmpcisewraunztcnbjwztammktjsfflgngvvqntmlsdcovkxv\
    qnxgybqbjsvvnelnxhwxxbojgnwrowlimgetrpbzpdqdsvoxgseusuamjgizqqcnmry\
    kwkuuazthrjfkehiogqsippsrggylllpkcyugslhywtgrfizsjyiufshiwulcgofkhk\
    qgefibnkglodiwuyozxzokspqzwyuwtawhcjgaelbsdlyrjtcrgbbxubateywtmiuxh\
    ewpbngqgyeiynwpfzwbffuqxyygpbnsmpqjjmiljzxrntodoyzihbhderzgyyahpukh\
    hsjsmarlciejoazfpwxmvqhepxzkqqyklglbisqvkxeqezrmxgpxrbpauzcihudcffi\
    kaoennstouwltrzgucnxvqypognvsqxqujzuwglncnszxwwciccfdsutgsoocokpoie\
    ymxbxvjmatfkonjmqpigbpfvkdpuwilnnofivystdrccdkwiuusuguimxkfhjedszhb\
    oehfhtaycurlyzxxlbsjxnzjknpnahoiezfhltuvyrrmltacsrvchlnakqbpuqpbpcf\
    xrapjuuhgobqvqrplzaowtzqgcvzxparghblwihjfmdmelakymzidymslrafdhrrtwj\
    acddniivtgewgbhkvzvocsfsuehxugsukgrybdiynuihsodwdlfkjtgpjbextdbjabx\
    gjvrgnozwikdjxwminopgncsnpghibnfzrsmbrbandudtrjapkzbukatorpiycqpgec\
    wwmezopdysggnhuwgmqejhflarbidjhljxoxppacgdexrhnvgtvrayspmhybrhhtcyf\
    qyrpbvcecbvocfuiroaydnwevngjuwzhojfgoeozkgmzacxvmovauybutgmjmiesevj\
    drakenvkyykykoogolgvojumtxznbthknlyuunqkcizhxvfvogjgnxbskjyughlgopx\
    yiqbsdqiijecxiqsstsicvxnobshgtomnzomhlqjlqnafipwnejirwmdkpnceoqpnrr\
    pntsydnvuvsqqbbppmuhnxhksgrxzmdiyjcdcbaemlnklldniaxioiepwdgbvplmbdc\
    ppxjkvagvdslmztgciinbcsrtjnrtvzcdexvjmpuycyluulrsebqrgorjozzmqoynfn\
    xoduufhwryagvcfkrxndgxnuldqaocpmhuzsxilcpwkhbpcwotynieuezobaogyekli\
    erfclnzjcojgnutcpvwklfonadujbswodtcgpowsaoudceqzskauxirwbdzfiyulmft\
    jmvcedtpqqkrqaraqwtcjmacnyhtibrcciwpttjszhvtsdcjtadfubnlqvcqtudkasm\
    hmyihrlmrlhradplgoshdgzzxqkyrnelpiilotohnlznowtbzdrlbraaijmnjmhcvro\
    arlubpbpyypqmqgrxckynyyqclnidwybiszqluufrkiteankefswfwfqbniuhywraco\
    ppthdentqzrojcummolhuwvwxfdsoadbimkutosqhnihhqedtlnhrvqfnubampcbuwf\
    qadsekfczypojvbtzmksmsjfeklmszxplqixzcvmgpmqhdehmmqyybjhasinexxswxy\
    zoodalkwnymfhzgtcyvrlfvpxorgeikzwsfamuqonwkvtkajnynjscczqvgkimngzzk\
    xzjqdydnzsqhzsmlocvewvcgilccuszzeeldbxuwpvgrralpigfscfisrgqbkvmzeiv\
    fviiyaakjprjwqkjqsnghwngjmxdvhrqzrojuzxkbqwaqarngbtfgnahyajpqknpwnf\
    mpqwppjgjcsbejlgskdhhqkkefxvsruburjpdvkeacbtdnmrnzkgciwzlvtmetbjjlg\
    ynhudqzruzfexqopgvawodhkisedjzjztvnkyzxcnraohfrxrntwyuuvxdarxzthzfz\
    ezpfcmkirijiwxaujebmwlpiutbyheipgewfuolbvqhkllrjkdhspzkhajwmzincyhc\
    gmcyjxlmahgieyypdzowzzghwrfpyjosnzkeaskqmzigxvnyaddnwuwyyeawwfmbolm\
    shdmlrmbwffrpzmqrkdevwtdwpwftqkakjtvpgqqzpebtzdrdcjlwhylbqjhcctcype\
    qtxfvfmjwqwuemlmtwscgbznlqipskwntzxdwfickgldmcnkfesunaxszkhnbbxzwub\
    plxzfhqaajhdsltimcyjdwkgsomurzorbjgqsyuzngmfwrbakdjhdruxijpngjzkdcz\
    wstzsldvapzwvjcagyiiukngbmieuoxfksuiudgmsawuejdvpvzrochvxkdllmgvyyj\
    wklzsisyrsaqohcolfwasmjnpsmkkbkgteozbbqigmpgsuuevudqrvzgjfcblmjejkn\
    xdsliexlumygsxxeovxyinrcuucrpwwsuphxqaqkfrjyoluoackaejhcatdsxehcqyu\
    itcsngjnkresouxajjavtgdvhavayegpcyilgthvhyzkxnxwosvcxnjnqkyxrxvdnan\
    xfbcisorgbgtlhxcbbptokbehmnmaicvjqqqvuyibqszthpnhgpvzqspmwkmmzffwlw\
    dsvdtvldzxskllurverhwyscljzpasmfshpypcczhnuycupmqxxlkihovxwgsmiteuy\
    viqujyoozcucsgkkpaztvirxctfvkrayhpaespfknfskotubenfwnsyhibbheafyzda\
    etwbthnltfaixnjccmumglhaotbayilsoqbzkqomwpeumknubhqpqqgxgtybnamitpj\
    zzkcuvrdrmcjzsosjlippecnqwssbesjigfxnosslewcrmhaotvqhayycfasnmvbein\
    buovkdykfjchvyrrnhqynzmhgjtdakkmylsgdyobzzpvcpovfkkdwjkyrmtgsqhcfxc\
    mbtcqfqgmtbwlhjxqbhpsysrravpuuphcpmavftyewcrpnzjcmzocynpalgxbjarrzi\
    yeyjjijcuogeoxszljkmvjeiymthuflxgmakcitajmowyvqutwrtkkvvcakuntbxdjb\
    czjfbzghotahbctgaubxnkipmbseflhpjuaycpctffyshkzolxcqmhvdaazvznuxewh\
    cxvranesvhijzuroeqedkuizgisadoicwrzrhgakthxbncpjhsimcyfimqxvycetdfo\
    ckpwdlqwwdrnoermqppigjxximxywyqkkwkwaucasnwcklwixlqwzqtjnwphikojrvj\
    qlojoaneedebegpfjftqkayyxpuhrefkhxirylecbneczhzrshwjlzhwxraxomeeoeu\
    wwbrzdfoapyrqtfnapgjlcraqkxknxmdzaeroqxxdfiojfhriglkznbicjvtkbiogzu\
    dnhbwyjdeyedkfhspitgraihcdacjmubvnjdnwanvzuxptapmvsshxpcedeexxresra\
    raugmyufvlgvsytyyoywyxyglliyunvyzjscfomcihbhhdjzgkgeqmmoljyharlqtke\
    bsynkhpchvmnaqowokkeyfbjlhgpltiadcimihsynghjukazexwlekcvmbysogtwyrm\
    dqltgjywncvryhilrykwklxawdsdirywbcxxchwjtqmwnzbxlinbmizoreloqlmmxvy\
    inxqooabhurdtjbmzbkitfcybxcuzxetqdzyhsxyfhuumrskczbhppdebwolijwxezf\
    huxchgzjlkpbvdtjlhzjwvessbacxoykbhpzsyuopqeygpjhvfpsrnucvwqektefyij\
    qwguyvhokhrclmfrcbqgfbwmptwlxxlevwijkzjoxyzpyrvpslgcluqqnudesgtmhiv\
    igoqvvauurobglszvgfmsvcpgvfsbrpxoxekanfnsgnaponjvskdbeedbvbqksmefpk\
    kwpyxwljogqulnemkjehkygehxiazmxnccappvohjdyjuaavoashzqzzunzvuphqiej\
    ldbszupmxzwmetmhvxfoekclksdycxmqvjvjlnbwyspzxsjchyzchlqmthjaieghcbs\
    hcdatxffralzxqxpgmzchakrtlmcsxfyjeybpuumtpjbxoapbybyvxpoxxndlfgdyhp\
    glxitnoxblqhynsftxcawajlrrhjzbbtfkabpokrwwylbtbxvmclikxhugbqhysmqry\
    bgougphtzynthoyeoimjhhobisjrrtxzobrrwmuabfyyrfaegivpreraccxfzfsaine\
    ogdlanjsekritjeimmhkfphgmefyvxvoffccdnxkkxulnbdwkvsyenaapohridltddd\
    lqlwhupoevrotahdplzgqczisxfrfxupyjllyujksarpsefnyxsepgeecwozmlnywvg\
    lliysvlexsdihjdtkzyfwclxbeeybggkihorqymdrozvcrrxkyvqpyjoglwscykbqbj\
    apcovxfssfeyvyergidholownvlyjwnnqiyxpqgtzrvplpsvzqugajgqgpimsvqtdob\
    pirwxtdrzqzsjaxnujyvteraailkwlokxwjdkkwefluplwnfstwmfftltbnfkusjuqy\
    szirderbngyegrsuhuovgmscttvqszyrstfokbkarkgrvwasvjusgnmuncekuzcszyi\
    zehuuckqnvxahpwdduqituewlisjqvictgmjzkochdmmrzmxmaovrhxlwjrqbsxbkwl\
    vfniuudjorfanzmzyimzdpbcgwcvjtfiyzvhkahacsornscrcfpeedptkojnpgdbgjs\
    sgdipvgwpepatqxuuoqfdesazbrlrmsosiihnteomconqelnjigyulrzoquezrgdsqu\
    dzadrtvtnajhmsssnulcuptbmjfnpoflwrpgxwdqzxzsrydggzqetiivylpjpdgoceg\
    zkdwhkmpkztsryfloalzgpfopwmgxyiibgglrwzddnbgjuicbrjenqfhqeubfbamudu\
    gybopzwflnoejothqcixgvmozhxdbijuhnfiphpdmojaepamgeirobagwtaiizwzdss\
    zzejwxppzqpotcsbkeixjzqjusxnmuzkfbymgahibeffuzbrvjslrxerijwsauaqqqf\
    vbwtondwdugthnzkoulgykqkgkyljgtmdgjstoklxqdkhfjolakirwodvqmxgjlxisg\
    kvpcmfuxuwzhltxorqvwwqjqyvfwvibgswsvnjwapsionfpkpsbqzkyzrwhbjlnvovb\
    lufmaoangzuuestyedciukuagcyqmlpctlnrhcfegcviwhrlrrwqbvrodaaiqezodbn\
    ctszmenvkxnmtqsotsusrinbobtnoguodrjcebzsteucsetmcoctlejowwpucqgyoct\
    bznssbozwcrrhfmggocefczcdpftxcksitfrheoyeohgmuueweyiqfmzlunqreoshjp\
    ndxjfihzfngbiyxtyydkpwdfcdhvxskncvyjundpsmpiiqjfkqsdxsuwzsgjbnzgdip\
    vwrhhsppxbmvwrdcllwmlscmqxzephuwbrlumclcmzeakinlnbpxmqtxrwajwlosxib\
    dqlyjwmhxlkbxvjqcghhyvfknporrzjfsmxwcdouzxwxbxnzhcpjnuledsnhpiuxtpe\
    tuxphvmgfpsitjzfvdivwqbgvncsgmbkjzwoseuhjzrilvqyezfagtxevzrsjbevsuq\
    qcafsntnayzboyoznucljywgqnochzdfqjqqtclvnaegsirqrdzbyozupzhfbtypytc\
    wxubeazndutrbclakmgszhhpgrwmzybuohhnfxoccknsuarupqacbbvhgtctzpsdtbk\
    gthttwvjzzqsispjlivflqpijdhjpkoumlssqrfvfrvkreubstrothhdqnlbkuavqxk\
    lcigrjdvpvrzxcnsgvvacgitswrxazcpmktrghzkcbccvpefzwuvrnbpikqahizyaeu\
    fgfkqlofgfgugugejtqsulxaxsrrwrteebumbbowihewbkwvkdubthpxwpwdbvsebxg\
    dovxzhkaypsvjandfwfuwhfqluszcqomlwsjrggbemcefttmffejosvznywcgbemosi\
    lcaqfehnmqqzbmjbxglzmsjxqdlppjpwajajjzayirgyxcprghelguxnpbhmkrraqjb\
    yipteuiazvjhhwyoumlazsrxanpmmvxdqecoelezgvvhqunpsdlipuoyfjzgbstiswy\
    izfennyrhfsdwsktgxjefzluidjdfyewmznagjitfbksljkblcidbmcvriykobjefri\
    jcdmgtmtrzcwzxjvcghppymy";

    #[test]
    fn example() {
        assert_eq!(Solution::longest_dup_substring("banana".into()), "ana");
        assert_eq!(Solution::longest_dup_substring("abcd".into()), "");
    }

    #[test]
    fn extra() {
        assert_eq!(
            Solution::longest_dup_substring(
                "dcsopfbhupztcyxctlyxocqwgcgydrxkbbeowdlkcehhslmidwphslbf".into()
            ),
            "hsl"
        );
        assert_eq!(
            Solution::longest_dup_substring("zxcvdqkfawuytt".into()),
            "t"
        );
        assert_eq!(Solution::longest_dup_substring(LONG_TEST.into()), "fsbrpx");
    }

    #[bench]
    fn string_1k(b: &mut Bencher) {
        let mut s = String::with_capacity(1_000);
        for i in 0..1_000 / 5 {
            s.push('a');
            s.push('b');
            s.push('c');
            s.push('d');
            s.push('e');
        }
        test::black_box(&s);
        b.iter(|| Solution::longest_dup_substring(s.clone()));
    }

    #[bench]
    fn string_1k_same(b: &mut Bencher) {
        let mut s = String::with_capacity(1_000);
        for i in 0..1_000 {
            s.push('a');
        }
        test::black_box(&s);
        b.iter(|| Solution::longest_dup_substring(s.clone()));
    }

    #[bench]
    fn single_long(b: &mut Bencher) {
        b.iter(|| Solution::longest_dup_substring(LONG_TEST.into()));
    }
}
