pub mod grid;
pub mod solver;
pub mod strategy;
pub mod translator;
pub mod util;

#[cfg(test)]
mod tests {
    use super::*;

    fn run_solver_on_bd(bd: &str, verbose: bool) -> bool {
        let mut solver = solver::Solver::from(grid::Grid::from_str(bd).unwrap());
        let mut iter = 0;

        if verbose { println!("{}", solver.get_grid()); }

        loop {
            let res = solver.step();
            if verbose { println!("--- iter {} ---", iter); }

            match res {
                Some(sr) => {
                    if verbose {
                        println!("{}", sr.get_name());

                        if !sr.get_to_place().is_empty() {
                            println!("Place:");
                            for cc in sr.get_to_place().iter() {
                                println!("  {:?}", cc);
                            }
                        }

                        if !sr.get_to_eliminate().is_empty() {
                            println!("Eliminate:");
                            for cc in sr.get_to_eliminate().iter() {
                                println!("  {:?}", cc);
                            }
                        }
                    }

                    solver.apply(&sr);

                    if verbose { println!("{}", solver.get_grid()); }
                },
                None => {
                    println!("--- end ---");
                    println!("solved: {}", solver.get_grid().is_complete());
                    if verbose { println!("{}", solver.get_grid()); }
                    println!("bd: {}", bd);
                    println!("--- end ---");

                    return solver.get_grid().is_complete();
                },
            };

            iter += 1;
        }
    }

    // #[test]
    fn bd_6_2389_test() {
        let bd =
            "004005000010900340080002009705080020000203000090050801300500090076009010000300700";

        run_solver_on_bd(bd, true);
    }

    // #[test]
    fn bd_2_0338_test() {
        let bd =
            "000060570320007040700410000030000400106000803004000010000094001010700059098030000";

        run_solver_on_bd(bd, true);
    }

    // #[test]
    fn bd_batch_test() {
        let bds = [
            "300000020064500000000901604010007300700000001005800090501603000000002810080000002",
            "302109508000803000000020000007406200400000009000985000030000050078000120009502800",
            "200010007000207000050000020005020400001549200300708001070804030000000000630000085",
            "900070008000109000000080000000207000830000057700608009095000170417000893002000500",
            "030000080000791000005000700004080500001204300200305008002508600000000000503010809",
            "000000000305904601040010030200000007007163400000207000020801070806000302009000100",
            "080060070200405009000803000600000008390248017000906000002090100006000400900000003",
            "300206007090407080001508900000709000030000060020000090900000002150060073004050600",
            "060004900040260017700900000600040350090302070053080001000001005510026030006500080",
            "060004000000200090204970000008400705040020080107006200000093607080002000000700050",
            "000943000000807000500000003006030100050701030004508600020106040041000780800000002",
            "709020000030906000200040003170000500090705080006000024900070002000201090000050406",
            "020070008000000430000504002800000029090237060760000004900803000036000000100020070",
            "000407000060205040010000020040609050003000200100000009200080005950000082004090100",
            "702010405050040020004000600570000061000000000000807000400103006900070008806504203",
            "394007000070000030206000009900208400080605010001709005400000107060000090000900354",
            "000802300400000050007000129090405001001000500600903040534000700010000002002304000",
            "000010800070640030300005000009050020540302019080090400000700002060031040003020000",
            "600500007008076000005098610093000006061000890400000130036740500000830400800005001",
            "700003080040060002000004090860140070400000005020097018070300000600080050080400009",
            "920000067000050000003000500100000004300401002500762003700000006000806000809040705",
            "208000104003000200760000095050308020000050000000271000030106040805040603600000009",
            "208000096130900020000062008003509040002080500040306200800290000060004052920000804",
            "300080007908000301010000050003204700000308000009705100780000035106050804000000000",
            "008090400600030001070000060007402600006010200030000040000278000000000000405901807",
            "005600070000003106001080040900035800000908000003140009040090600509300000020006700",
            "204003901060002030100079004652000000001000300000000245500380002020400050903200806",
            "601043008080605010000000007460020030700508006090060051100000000020409070900850104",
            "600030009000710600027800400000000160290000048075000000003001920004089000900060004",
            "600010007030050040800206001020000010000301000003702500901000803000803000050000070",
            "005030008000270000823900007380000002000407000400000095600002754000053000200040800",
            "008000500020805090000106000705000203060070080000903000510307028002050300030000010",
            "560000023000000000030201080040000070000964000003708400250090047079040160008000300",
            "002090000070800050001025407003000020607000305090000600408570900060004080000030500",
            "200800001005090600090600080000308402040000070803209000030001050006080700700005004",
            "001038450000904160000000302010000006000281000900000010104000000095407000086390200",
            "008060200035000160010040090504000702000583000000704000160908025200010009000000000",
            "030100047000207000078000030350000000007080200000000096090000670000509000160003050",
            "500002096290600300000004010086200000000040000000007960020400000009001034350800002",
            "600470000080002000000000614024900008907020503100004270841000000000100020000098001",
            "000030008000004160080006009005710400003000600004098500100300070057200000200080000",
            "500400632621050090300600080000000503060000040703000000030002008040080379875003006",
            "030700000000000091010056030500800904006040300304009002020930010860000000000002050",
            "003620100200003000047010000700400060090000080030006007000060490000700005005031700",
            "010800000000019006007006280436000700000000000002000348073100600500380000000005030",
            "100060008003001400094003560071000000800020009000000180068300910002900700400010006",
            "900600738003700005007000600002030000380000049000010300001000800600004900438007006",
            "000609008504000000920003000490000005805402103700000094000100087000000302200805000",
            "000600000004010200058200630000401507070020040409507000032008910001050700000004000",
            "000000600042890030500201040001302760060000050027608300070504006080063570005000000",
            "100037098080000200009000006000004503000758000407300000700000900001000060620410005",
            "003000400002904500000203000090000020254000768800000004000501000570060091000080000",
            "000050070800062000007400600040218900750304068009675020006003800000980006010020000",
            "004300000070019020000870005030000209052040830701000060200058000060730080000002700",
            "008000060502810400090000071000102030030080010020905000680000050003069108010000600",
            "005600000740018300010000008500760090400000001090045007800000070001870053000006100",
            "007203060100090000003500908700000306030000080902000001301007400000040002040102600",
            "300400906001063800460020050040000003023090480600000090030070068007380100806002009",
            "903106805140503067000000000002304100060000080004000600807000302000807000050000090",
            "800104002900000003013902480000518000000000000500609001302040506000207000100050009",
            "000106000100032006090040000002004003037000980500200100000070050600520004000309000",
            "600153800001700000300008000080007060570080023020900080000600008000005300008319004",
            "001000500040005870895000103060030000000459000000020030107000964029600050003000700",
            "060350020200080405075004300008000004350000082700000600007900860501060007080073040",
            "000000000104000907060000080070935060000408000005070800830050072507000103006000500",
            "300040200040005000000200048296000005400000009500000826750008000000100060001030002",
            "000070000901000304006809200000682000000000000008315900009000700002060800850000031",
            "000089104210067090000001020005000003900000008300000200090600000080370052504910000",
            "000723600520600300000000010000000260460000087091000000050000000003006071009214000",
            "480000005005000607030026010009802000003050200000307500050170060601000700700000042",
            "038000540005000600600000002000403000000928000050701030067209810020080090040010020",
            "920308015000000000530000047000020000090000070002605400017060250080000060200704001",
            "001930600000041980000007001500700090043000870020003005100200000095310000002059700",
            "200539610000000095000018000039000000010050070000000120000180000970000000063497001",
            "120504089008030200040010050000902000080000030005080900000000000570801024002070500",
            "800205007070809040100743008920487051000000000001000400007000300000601000410000095",
            "000050000050924030400000006001040900640000052509000407000000000300602005080709020",
            "205010708041000260607000405070000020000781000100000007000802000090406050004000600",
            "910060005000001702072000300060402000400080001000706040004000950706900000500020037",
            "000060570320007040700410000030000400106000803004000010000094001010700059098030000",
            "080000090002060400040708030000070000004090600006205100060000010030050020400803009",
            "002894500000000000006307200001000400030000050490502086070000040900408005100000009",
            "800060009000000000000283000020000090360401078000958000010000080046000230080692040",
            "004005000010900340080002009705080020000203000090050801300500090076009010000300700",
            "000640007070000620600700000020000340096321870035000090000005004062000080100087000",
            "007004000001300000946005803300000700054000280008000009109800645000002100000500900",
            "100000006063000270400050003030502040008736500000090000000000000200000009700964001",
            "500040006070000040000901000040030050000000000006000300003109700900704002007863500",
            "000080000020194060000236000500000001603000204010903050701050908300000002060000040",
            "001003080093100050700000060000300016000050000960002000070000004010004690040800200",
            "000490300200000000034000002510029400007010900009740035400000750000000009002058000",
            "020700000005409002000052040054000016006010700710000950030620000900507100000001020",
            "201000003003050200070000054000593000030102090000867000410000070006020800700000409",
            "500003002060002080007690300310000200004000700008000013005061800070500030100400005",
            "050000020004020600001406500030000070065000130000643000940000067020080010000702000",
            "600000040009508300021400000002004065040000020750300100000003570003905400010000009",
            "063201890000050000072809310005467100000090000020503070306000407050000080004000200",
            "500000008000090000006010900000301000002945700070000050400000009013080470069000120",
            "610000039000902000004000700200709006000000000706010502100000007507030608000208000",
            "000020000900060005032000940000090000305472608090000010000000000083000720040308060",
            "700804003209000708030070020000030000020108090100000005073090860500000009000263000",
            "200080065000230900000009004000300508810000079507002000400700000002051000730020001",
            "800019020520000089000400070040000006001080700700000090030002000150000037090170005",
            "007035680800900000300006007503090070100703006020050109600300008000007004071840500",
            "009062480002000060000410000050607000600050004000309070000036000060000100035890200",
            "019000650500000003008907200090000040102000307000030000000805000001040900030010070",
            "300608000600000000425003060004709008000060000700501300070900543000000002000307009",
            "000956000070012080005000900140000003950000068700000012008000300060830020000297000",
            "000485000500206009070010020600000003200000004007648200009000700001000400020507030",
            "008001047050020031000000000509078003000602000200590108000000000460010050980300700",
            "004070200000010000080904030000105000045000790200000006002000600170000089090050010",
            "000000300000002008720060504370020000001603200000070093803040076200100000009000000",
            "001400090500000200982037100100300000803050902000002003008760329009000004070003800",
            "060001000002045000040030160080010700920000013003070050098060040000190300000700080",
            "300005100000040200107000050630072000000806000000950031060000402008060000005300009",
            "800702003070108060200000004060000020900000005030829010080000070005080300009503600",
            "000504000012080950000010000700000002500000008043000670005897400000060000980401037",
            "001008030020690000000500040107000680000020000092000705010006000000075060050200400",
            "040000070008060100926010853005906700007148500800000006070000030003205900000000000",
            "000000000107308904400090002000734000000000000240060073760000018000000000300821007",
            "840050029000102000000000000009000800300090007160307052750000083010070090908000604",
            "060958070007000800054306290600000005048000710700000008000020000000605000300814007",
            "010000070006000100509040803000623000000000000380905026800704002001060900900030004",
            "006309100700040009010000040000738000500000008480060021001402700000803000900000002",
            "760000001003007092002000705000871003000502000400639000106000300270300100500000078",
            "060802090580000031009301500090010060004000100000705000200080007008040200030000040",
            "060000030014000860005000700046708950000020000032506180000000000000103000500872006",
            "960040012010000030004000700000402000000109000030050060053704190008905600040000070",
            "002000800070000050005721900013000540600508009007090300000060000000207000700803004",
            "030000040005000900800050007000000000304962108010485060080000030740806095000030000",
            "900201008000000000200603005000000000097030560080709020000000000310904086400508009",
            "392800001000001006001030208070000004009040300800000060907010600600500000100009753",
            "060102040002304700004000500720000054418000972000000000000596000000000000390000086",
            "903000700000041360000300040000060180100090007035010000010002000064180000008000402",
            "000000000800090007400502008000000000004103800002856900000020000270000084945000321",
            "090345070000020000000109000021000560600070004409000307000000000500804001803507406",
            "000003170000709050000465009607000030800000002040000705100654000030802000086300000",
            "070000600008200009250010000013000000700806003000000870000070065400009100002000080",
            "900512008000000000200396004008000300060030050300000009150020097820000065006000800",
            "800749003020000010030080090001000200079000830200000005003204900000000000700603001",
            "000056098005400010000800020300067400006000800002180003010003000030004100290510000",
            "024090003780030000000000800007640000901050708000087900002000000000060072300070610",
            "000239000500080007001070400070000050010000040002908700200000008060000070005746200",
            "100000005003901600800537001006852400080106050900000006470000068010000030000010000",
            "400090008002040700000301000507000103304702506020000040000000000140000079000165000",
            "000409075708010009090000030000370000140050083000064000060000090400030607570106000",
            "650100003007000006004600180000702809000040000701809000026008400800000600100005098",
            "050007004090200100003004200080062030000000000040390060008700400004003020500800090",
            "000009430320600050405080200500090080001708600080060005007030504050006098043900000",
            "700050902002000500650100070000925700200301009001847000040006058006000400507090003",
            "072031000108070690000600000000190400209807301006043000000006000064010805000750160",
            "980030057062000940070000030006123700030000060000080000600000005010675020000892000",
            "026000930530080026000000000000000000740060083008501400070010040002000300409070208",
            "016500040700010000040608900150000400000201000002000087001702050000030004070005810",
            "000000000096700340015004260009206080000040000020309400081600920052001670000000000",
            "009300408074001000300490050400000029000000000180000005040023001000100630501007200",
            "070590400000000000960710805000180090080965010010034000408051079000000000001076040",
            "020009000010008657060170000650000200009000400002000061000026030231500080000800020",
            "030090000000020570100004300800300040500207001070009003004600008062030000000050060",
            "006040000004700008001206000069001000070953060000800390000102700500004800000030100",
            "020907040500000007000486000800103004006000300007804500000000000405000701703020408",
            "000960032900070050000502000009430010103000204080027300000603000030080005820059000",
            "960000081000000000000489000890305027030000090050000040600203009001050200040608030",
            "000605000030000080070103040000000000007904500069000130720508096800060004003090800",
            "009618500050000080003090600002000700400000003600204001007000100900702005000301000",
            "000090208006207000090506010281000700060000030007000826050109080000805300108060000",
            "700000003050000020109246705070010050000000000060802030000381000000000000006070900",
            "010056400500000300000800901469010000000070000000090146306007000004000003005960020",
            "009000400036000570400000008000050000090671050085030140007090200000000000050864090",
            "806050103400306002030000050600904001100020005000000000010602080260105034000080000",
            "000470000024005900005930600400000060601000702090000004003048500006500310000067000",
            "070000010500020007000796000030408020807000603000000000005010400062903150100805002",
            "070200100008005003205009670057000004000090000600000730041300507500100300003004060",
            "290000700108604000004090000007006042086000530520700900000070600000401309009000028",
            "007301600000050000320000017006839200730000091000000000803040106090000050100080003",
            "030010000021078000007200000800009520006000900095600001000004800000160430000050070",
            "030910400005030002400005030003000005180000076200000100060400001500080900009051080",
            "309672000001000000760040000056000700400080002003000490000090023000000900000321805",
            "000070026405009000030810500020000400350000072001000050006095040000400201840020000",
            "037020890800070004050000070200090006003000400070060010000904000009107300010050040",
            "007385000005000800090004075609030002100206008700050103840500010006000500000643200",
            "600090087800004600079000400020605000300070004000308060002000890008900003790030006",
            "090070000038600094006098230002000060804000309070000500089740600160005480000080070",
            "009041000000600180000708009004000760500000001061000400600205000017006000000430200",
            "500060089000703100000500040009037400100000008007910600040006000001409000390080004",
            "007286400009040200000109000400603005050000060000020000800000004700060003620090018",
            "000010000100804006098020540000060000040000050852000674400000007037000460006709100",
            "805000109001000200020060080007050900006103500004807300009000600070405090010000030",
            "090314700800000060040008100002007900010000070005100200009700030080000007007863010",
            "002000310003040809100060000500008000001309600000600008000050007308090200025000900",
            "007093050301400008090002000108000200700205009009000603000600090600007501010320700",
            "000069380002800060806047000075000000600090004000000610000480102050002800084930000",
            "000108000000452000408070201701000506600000008003020100100000005060509010050080030",
            "071000320000030000600809007010000030004705800006000700040603070037208960900000003",
        ];

        let mut solved_count = 0;
        for bd in bds {
            if run_solver_on_bd(bd, false) {
                solved_count += 1;
            }
        }

        println!("RESULTS: {}/{} solved", solved_count, bds.len());
    }
}
