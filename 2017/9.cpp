#include <cassert>
#include <iostream>
#include <functional>

const std::string PUZZLE_INPUT = R"({{{{{{<!!u!'a<e!!!'!"!>,<o!!!!o>,<oio{>},{{{<,!>o!>},<!>},<!i!{,a!!}!!!>!>},<!!a'>,<!!!!!>>},{{<"!!!>'!>},<!>},<!,!>},<!>eu">},<!!!>ua!>!>!u",!}u!,ieu'u{{a>},{{},{{{<a!!o,o<>,<e!>},<!>iiu!>,<!>{!>,<>},{{<!>,<o,!!!!u>}}}},{{<u!'!>!!!>},<!!!>e!!!>!!a!!o<i>},{{}}}}},{},{{{{<!>,<o!>{oi!>,<>,{{<u!iae}{!!"ia>},{}}},{<!>,!o>,<">}},{<!!!!!>o!>},<{>,{<!o!>},<aa!>},<uu>}},{{<}i!!>},<}!>},<!!!!!>},!!!>},<}a!!!!!>!!u>}},{{<!!o!!!o>}},{<oue{!!!!{",,!!o>,<a!>i,!!!>!!u'"!!<>}}},{{{{}},{{{<!>{!!"ie!>!{!o{!!}u<}"a>},<',a}<'a!>},<>}}},{}}},{}},{{{{{<"u>}},{{},{{<!>>},{<!>,<"!!!>!>},<!"!i!>},<!>},<!!ie!!u>}}},{{{<!>{o,e!!!!!!!>,<'!!!>e>}}}},{{},{<!>},<o'''!!!!u!!!>!!}oe>}},{<!>},<!'!>,<!!!!!>!>},<!>,<!>},<{!{!!!>!!<}!>>}}},{{{<o{i!!!a!>'!>,<,!{!>,<!!u"!>a!>!>!>},<a>},{<!!i<!!!>}aiu'<,!'{!>,<>}},{{{},{{{{{<!!!>},<!!<!>a,!>,<!>,<!u!!!>>,{{<!!!>>}}}},{}}},{{{},{{{{{}},<!>,<!!!>i!!,!{!>,<!!!><!!"e!,iua>},{{<!!e!u!>!'u!>,<!!!>!!,!!e,!!!>!!ui}"">},<!",>},{<!>,<}"{!!{!!e!>!!!!>,<>}}},{{{}},{{<,ee!a},>},{{{{{{{}},{<!!u}!!>}}},{}},{{},{<!}!!,!>,o!!!!'>}},{<!>,<ee!>i!!!!!>u,,,>,<>}},{<!"<!>,<a!>!!!>!!a!>,<"!!'u!!<!!!!!>!>},<<>}}}},{{<e!><!!,<"'!!>,<e!<}}>}}},{{<!!!>u!>,<io!<i<i>,<!!!>!!!!!>!!!>>}},{{{{<!e!!'i''!!!>},<!!<}>},{<!!!>}{'u!!{!>,iu!!!>o>}}},{{{},{<ui!>,<!>},<!!a}}!>,<!>,<},au!>i!!>}},{{<e>},{{<<!!!!!>}!>,<u{'!!{i>,{{},<!!!>},<{!>},<"'!>},<<!>,<u>}},{<!!!>!,!>},<",!!ee!>,<!!},,"o<o>}}},{<}{"u}!>},<u!!!>!,"i!}!>,<!!!>,!>,<{i!>,<>,<<>}}}},{{{{<!!}!a!!"}{!ioi>}},{},{{{}},{<'u!>!>},<!"!!!>ie!>},<e!!'<,}>}}},{{<!!,}>,{{<!a!>!>,<!>!!{i'"{!>,<}!!!>!!,e!!i!!!>>}}}}}},{{{},{<!><!}i!!!!u<!!!!a!!!!!!u!>,<,!a>}},{{},{<}u}!>}!!!>i!!"!!e'e{!!!>},<>,{<!}}}!>},<"!>,!>},<o,,e!>,<!uu>,{{<!>e,!>},<,u!"!!!>o!!"<!!!>!!!>>},{}}}}},{{{},<!!!>},<!>,<o!>{{!>,<}"o!>,<uo>},{<!!!!,o}<!!,!>,<>,<,ue!!{!>},<}>}}}},{{{<!>,<{!>}o>},{<e!!!!!!!!!>!>,<!!">}},{<,a!>,<!{}oa"'>},{{}}},{}},{{{{<!>,<,{!<{,!>i<!>,<!!<!iu}!>},<i!}>},<!>,<'"'!>'!!!>!>},<!>>},{{{{<e!!{!>},<>},<o!!u!>},<'!,!o!>i}{!!"!!!!!>!e!!,!>},<!!!>,<>},{{{<{""i>}}}},<!!'}>},{{<i!>e!!!!o!>'}"e!>,<!!i}!!<u>},<>}},{{<uoei!{!!!i{ei!!!>!>,<,!'!}!>},<!!e>}}},{{{<a!i<'a}"!>},<<o!!!>!!!>,<!!!>>}},{{{{<!>,<uo!,,,!!!>!!!>!>!!!!!!oa>},{}}},<!!!!"!!!>!!!!{!>,<'!>},<!"aa!!>}}},{{{{},{{{{<a'}!!u<>,{<>}},{{{<o!!!!,a<!!!>!>},<"!o>},{}},<}"a!">},{{<,u!o"o}!}'!!{'!"!!!>!!u>},<i,>}},{{{<!>,<",!!!>!!!>u}!>},<}!!o>},{{}}},{<}<uu>}}},{},{{},{},{{<ui!!!>!!!>!!<u!<<i!>,<}!!u!!!>ou!!!!">}}},{{{<!!!>!'!!!>,<!!}!o!!<a<ia,u>,{<'"}u<e!!!{!!!!!>,!>},<<a>}},{{{{<<,!io!>,<u!>,<{!uu!!!>!>},<!oio!!<u!!'>}},<ae{}<!!!>!!!>!iaa!>},<!!iou>}},{<!!,!!!>!!!>e""!>},<"!!!!!!u,>,<!!!e'!"<!!o!>},<'!>,<!>!>},<o}ea>}},{{{},{<ii"o",!{,"!!!!!>},<>,<!>,<u!!o}<!>o!>,<!>},<,>}},{<!!!!!>,<i!!!!{,>}},{},{{{{<{{!>,<!>},<a!>,<>,<}!!{!>,<o!!e!>,<u<>},{<i!!!>,<o!>e!!!>>},{{{},<}!!{!!i!oa!!'!>a!,'!!<'!!">},{}}}},{{}},{{{<>},{<o!>,<!"u}!!>}},{{},{<!!<''e}a!!<ua>}}}}}}},{},{{{},{<',}!!!>!>},<!<{i{!>},<i!>!>,<!!'}'!!u{!>>}},{<"}!>,<,a<u!!oe!!!>,<>}}},{{{<}!!!>!>,<!>},<i<!>,<au!!!>},<">,{<i>}},{{<!!"}!>>}},{{<>,<,a!>},<u"e!>!>,<'!!!>'>},<!!e!!u'<}!}o!>},<oe!u!!!'>}},{{},{<!><o!!!!!><!e<"!>,<i!u<!>},<o>,<{!!!>,!!e!!!!e!>!!!>aoe!!"u{i>}},{{<a!>},<!!!!u">,{}},{{{<o,>,{<!!!!<>}},{<,>}}}}},{{}},{{<!ae,>},{{},{{{<},'!"!>},<,'oi>},{<!'!>},<a!>},<!>},<"!{!>,<ua!i!u!!"!oi>}},{<!!"!!!"!>,<!>,<}!>},<!>},<!!{e!!!>,<>}}},{<,o,}o{!>},<,!!!>},<oo,e>,{<!>u!>{}}}!>,<i!!!>">}}}}},{{{{{<!!!!!!!>e!!<!!!!!>!'!>!!!>'!!a'a{!>},<u>},{<!!!!!>!!!>,'o!>},<!!'oi>,{{<!!o'},i',u'!!a<"{!,!>!!e>},{<!!!!!>,<!}o{,>}}}}}},{{{{{{{<!>},<a!!'a!!o,!>!!!!uo!!!!!>!!!>>},{<<u"{!!!>},<i>}},{{<u!>e!!!>!>'!!<!!!>ee!>},<!!!>{!!!>io!>,>},<}>}}},{<!!!>}aa{a!!uu!>>}},{},{<!!,!!e}o}!>}!>,<>,{<a,!>},<ou}>,{}}}},{<iu>,<>},{<!>},<!!oo!>i}>}},{{{{},{}},{},{{{<}!!!!!>!,<i,!>u!!!!}{"!o!!!>,>,<o!!!>!!,!io>},{{<'!>!!i!"!!"!>}a!!!>},<>},<{'>},{<'!!,,!!a!>},<!>,<{a,!!>,<!>e!>},<'!!!>!!<u!>},<!>,<!>{!>},<!!i!>,<!iie>}},{{{{}}}}}},{{<!>}>}},{{{{<'!!!!!>{!!!>},<o>},<>},{<ou'{!i!>>,{<ai,eu!!!>'u!!!!!>!!!>a{,!>,<a'i>}},{{{}},{<!>},<!>'""!'!!">,{<!>,<{u!>},<,ia!!!o!'{i>}},{{{<!!ee!<!!!>!!!>o>}},{<{iioa>}}}}},{{{{},{<!!!>"!!!!e!>},<e!>e"}!!}'!!>}}},{{<>},{{<}!!}!>!>,}i}!!!>",!!!>"!"!ai>}},{<!!,>,<!>,a!!!>,<!>},<!>,<!>},<e,{>}},{<,!>,<"!"!!!!!>},<u{!>},<ao>}}}},{{{{},<e!i!>,<<}a!>!!a>},{{<<!!iu!>},<a!>},<eu!iu!!!a!>},<o!>,<"!!,o>,<<!>},<!!!>a>},{{<,!>e<>}},{{},{{}},{<!>!>!!i!>>}}}},{{<!>!>ai!>,<!!!>>,{{<o!!!!,!!!!!!!!!><!>},<{!>eu!>},<!!!>!!<i!,!o>},<!>},<o!>!o!!,}!>},<!!a!!!>!}e!!o"!"'u!,>}},{},{<}"!>,<!!!>!!!>!e}!>!!e{}''>,{<!>ae!>},<u!!!>o!>},<!{!>,<!!u<"!>},<>}}}},{{},{{{<}!>},<!!oa!!i!>,<!>},<!>,<!!!><u!>,<'>,{}}},{{<!>!!,<,',!>},<>},{}},{<<!!!>}u!!!!!>},>,{{<!!"!"'>,{{{<iu!!!!!>},<!!!>>}}}},<>}}},{},{{}}}},{{{{{{{},{<'{'{!>,<!!!>!!!>u"uao{a!!!>u,i!!!>>}}}},{{<",!!!>},<">},{{{<!>!>},<>},{<!>,<!><!>},<!!{"<uoi!>,<e>,{}}},{{}}},{<!>,<>,<!>},<ae'e!!'<!>,<!>!>}}!>,<!>},<u>}},{<}i!!!>},<u>}},{{<!}!>o!!!{}!!!>>,{}}},{},{{{{{<>}},{{<!<,a!o"!>},<!>},!!!><!>},<>},{<!i!>o!!!>},<!>},<e<{e!>!<!>},<">}},{<a<,!''!>{',u,}u>}}}}},{{{{{{{<u!!!>o!!,e!>!!ai!!!''!>,<}!>,<!!!{>}},{<!!o>}}},{<u!>!>,<}"!!>}}},{{{<ui>,<!!!>eu,!!!>oi!>,<!!!!!>>},{{<!!"'!>},<"!!!>!!,{}!!'i!>a>},<!>},<!!!>eo"i!!o>},{{{<!>},<ui!>!>,<ie!>},<uu>},<o!u!>"!>},<e!>"e,<e!>,<io!>},<ai>}}},{{<!uo!!!"a!!!>},<"!!!>o<'e"!!'eo,<!!!>>,{<"ua!>},<!!<!>,<,i>,<!!!>'!!!}e{!!!!{<eu!>},<!!!>!>},<,!!'a!>>}},{{{{<!!!>u}'!>,<ea>},<!!>}}},{{{{}},{<!>,<!!ao!!ae>}},{<!!">}}},{{<!>,<ie!!!>}<i!<!>},<a>,{<},!,"o!>,<e'!!!>a!o!>,<'>}}},{{{{<"!!}!!"}!!oo>,{<!>,<{,ao<"a!!"{>}},<{'"""}!>},<<!>},<<!!a!!>}}}}},{{{{{},{{},{<>}},{{<!!!>},<!>},<"{'a{eo>},<!><!!uiaa,'<a"!>},<!>,<<{"o>}},{<oa{,!!!!!>,a>,{{<a{<i}e!!!>{>}}},{<!>'e!!e>}},{{{<!!!>,<},ae<!>!!!>"e!!!>>}}},{{<>,{{{}}}},{{<!e!>,<a<a'!!i>},<"{!>!>!!!>{o,!}o!>},<o!>,<!!!}"!!>}}},{{{}}},{{<'!>,>}},{{{<!>},<!!!u!!o,!!"'!!!ae"!!!>a>,{<!!{{{!>!}!!oeu!>u!!e}a!>,<e!>,<,>}},{{<!>},<e"}!>u>},<o'i!!!>!!o!uu!>},<,'e>}},{{<e!!!>!>,<<"",u,>},{{<>}},{{},{{<"}!>!>}!!!>!!!>!<u>},<!!!>"!>,<ae!>i}!!!>oo,!a!ue>}}},{{{<>,{{},<}!!<>}},{<>}},{{},{}}},{{},{}}}},{{{<!>},<oiuu!{!>,<!!{>,{<o"!!<!>,<i,!>!>,<>,{<{!!!!'!>},<'>}}}}},{{{{<<!!!>!!!>>}},{{<u!>,<i!{a>},<!!!>o'!>,<!>},<<!!!>}>},{{<!!iie>},{<!{!!e}'i{!>}!ou>}}}}},{{},{{{{{},{<!!o<i!}!}ee!!!>!>},<"!!!>},<'{'e>}}},{{{<o}>}},{},{{{<!>,<!!e!>,<>}},{{},{<!e<!!!!}u!>,<,ui!>},<i,u}!">,<>},{{<"!!<}!>u!!i!!!>,!>,<e"io'<u>}}}}},{{}},{{{},{{}}},{{{{{},{{<u!>},<!>,<i!>!>,<'a<>}}},{<aa>}}},{{{<u!!"!!'!>!>},<i{u!>},<'>}}},{{},{<<ei!{!>},<>}}},{{},<,!!!>!!!!!>,<!!{'}!!!>,<!!!a!!"!!i!!!>i!!,,}{!!>}}},{{}},{{<!!i!!!>e!!!>,<!}!!!>!>,<e!!o,,oaa>,<{o!!!!!>i{!>>}}},{{{<u!!}a"u"a<}o{!a>,{{},{<!>},<oo!>,<i,!>u!>u!'!!"ae!!!>,<i!!!>>}}},{}},{{{}},{{<!!i<!>,<!!!ao!>},<!!!!!>,a!!!!!<>},<>}},{{{<!>,<!!,!!a!>},<!!!>},<!!u!>!><!a!!!>}!!oe!!aa>,{}},{{<!a{aa'<!>,<>,{}},{}}},{},{{},{<ao!!!>!>!!",e!!!>a!!!>!!!>u,'}>}}},{{{{{}},{<!!!>,<i,!>,<ei<'u<<!!o}>}}},{{{}},{{<!"a!>},<>},<ua"!!!>a!>},<!!!>'!oau>},{{<,!!!>!!!!!>,<{u'!>,<'!>'!!!>,<'!!<{>},{}}},{{<e!''!!!>ei{o"<!!o!>,<au>}}}},{{{{{<!!{}!!!>!>!>,<!>>},{<'!>o!>,<!!!!>}}},{{<!,!!!>},<i!>e}!!<a!>,<e!!o!!<"io>}}},{},{},{{{},{}},{{{<,,!!!>{>}},{{<!>},<i<e}!!ou<!>e>}}}}},{{{<,!>},<!<<<!>,<!>,<}!!e!!!>,<'!!!a<}!}!i">,{{<i!>!!!>{>}}}},{{{{{<!!ae>},{{<>}}},{{{<a!!!>,<!!!>i>},<i"}!!>},{{},{{{<'<!!"!!!>,<!>,<!<!!{}!>!!{ao!!>},{<u"u'"'o"!!}<!>,i{'!}>}},{{<e!!,!>,<!>!!!!!>,<u!!!>"ei}">},<{"a>},{{<'!!!>i}i!>},<a!!!>,!>!>,<!>,<>}}}},{{<!,e!!!>oe"ia}!!"!!!>u!>!!>},<!!!!!!}!!!>!!!!!!!>a!>,<,!ouo'e!!!!!"!!!>,o{o>}}},{{<}o!!,!!{ao'e>},{<!!!>"!!!>!>},<!>},<>}},{{<i!!'a>}},{{{<<}{!>},<!!!!!>'o!!'!e!!"!>,<o!>,<!>,<!!a!>,<"">},{{{<<""!!>},<}{e}<aiu{!>i'e!>},<>},{{{<>},<!>,<'!!ue!!u}<{!!!>{,}!!!>!!!!,!>},<!>,<<<>},<e!>},<!!!!{!!!>!<e>}},{{{<"!!!>'!>!}!!eeo!!ui!>>}},<!>,<!>},<>}},{<!!"{>,<!,}{!eau"!>,<!!"!!!>u!>,<"}!ee}>},{<,a!!u>}}},{{{{{{{{},<!!!o,u<"{"!>!>,<!!!>"!o!!'<o}!!!>!!!>>},{<!!!>ii!!ie!!!>!!!>},<u'!!'ei!au>}},{<!"oi!!e!!!!!>e}u!>},<!>},<!!!>!!!>{'o'!!e>,{<oo{!{i!!!!!>!>},<e!>>}},{<!o,>,{}}},{{<eao<>},{<!>,<!{"}>,{<i>}}}},{{{{{<!!oo!!!!,!!}o'<}u>}},{{<!<"e}!!!>!!!>!>o!!!!!!<e{e>}}},{{{}}},{}},{{<{e!>ie!>,i!>},<"<<!!"ie!!>},{<!!,!""!>!!>}},{{{}},<oo!!!>o<!!!>>}}},{{{{<!>'<!!!>e!!!>,<a!>>,<!!!!a!>!>},<!!e'ao!!!>}>},{{}},{{{{<!>,<!>},<"!!},>}},{},{{{<{ioo!>,<','!!'!>},<!>},<!!!!u>,{<!>},<>}},{<!!!>,<oe!'!>!>},<}"i'}!{,"!!!>>}},<io{aui!>a}!>},<'e>}},{{<'!>!>{<!>,<o"}i!>!>},<<>,{<!>,'!>},<!i!!!e!!!>!!!>!>,<!>{>}}},{{<>,<{e!>e!>{,{!!!>e{'a>},{<!!!!!><}!!!>'ia'">,<!>,<!!!!eoee!!!>",eoe>}}}},{{{<!ao!!ee!!a>},{<},<!>,<u!>},<u!!!>>,{<!!!!!>!!!>i!!!>!i,"{o>}}}}},{{{{{<>},{<!>!>},<"oa!!i!>},<!>,<!!!!}!!!>},<!!!>!!,>}}},{}},{<>},{<<!!!>!!a<">}},{{},{{<o!>ee"!!!>!>o!>{u!>!>,<'i!>},<!u!>!!>,{{}}},<!u{{}!!!!'e,e!>},<>},{{<!!!!},<!!!>{!!i!!<!>,<>,<!>ae!>},<u>},{},{<<!>},<e!>!!!>,!>,<a{'!>,<"i!!!>a!>!!>}}},{{},{<!!!>,<>},{{{<!!!>!!!>,<!oeio{i{e<!>,<e!!!>!!!>"!>o!!!>,<>},{<!!,"{}!!!>'!>"!>},<>}},<!!u!!!>!>,<o!}>}}},{{{<<!>},<"i!!!!!i!>},<!!<!u!>},<<'!>""i!>,<!>,<>},{},{<eu!>!>,<!!!>,<!!!>}}i!!!>},<>,<,<!>},<{!u'e!!!!i!ii!!!>!>,<oi>}},{{{<>},{<<'}!>,<a{,"!>!}!>!!!>!!!!>}}}}},{{{},{<a!>,<!!u!>!{!>,<>}},{{},<{<!}!!!>!!e!!e!>!>'!{,,>}},{{{<!>!!<<!!,!!!>,}!>},<!>},<uu!>,<!!>},{{<"'!>!>},<<!!'}!a>}}},{<e!>,<}}!!o"'u!>>},{<!!}!>,<e!!a!!!!'eu>,{<,!>,<!>,}}!>,<!>},<!!!>!!u>}}}},{{{{{},{{<{e"ou!!o!>o'{i>},{<"'!>},<!!!>!>,<i!!,<i<!>},<i>}}},{<!>,<}!!!>!>},<!>{e!!<!!!>}!e>,<}!!!!>},{{},<a,o'{>}},{{<o!!!>{'!>!>,<}}a{}i'!oi!>},<,!!i>,{<,!!!>,<{!uei{>}},{{<uau!>,<!!!><>}},{{{},<>}}},{{{{<u!!!>},<!!o!>,<u!!!>a!!e!!!>!>},<e,!>,<,!!!>!>!!ei>,<!>,<!>i!!!>!!o<''{!!,!>},<e"}!>{>},{}},{{{<!>!>,<!>,<oui>},<!>},<>},<!u!!a<!!,"!!!!!!o!>'!>!>!>},<>},{{{}},{<'!!}!!'!!!>!!u!!!>a>}}},{{{},<<{},!>!>},<>},{{{<'!!!!!>,<}<{}e!!!!!>!>,<>},{{{<!!<},!!!>!!!>{!>,<!>,<i>}},{{<'>,{{<u!>},<"!!!!!>!!!}!'>}}},{}}},{<>}},{{{},<',i!>},<o!!!>!>,<!!!!aau"!>!!!u!<>},{{{{<e'!!!io!!u!!!">},<e!!'">}}}}},{{{},{{{<!!!>!!!!!>>},{<{,!{{}{u!!!!!><{ou'>}},<'!>!!!>u}!>!>,<a!!o"!>>}},{<!<>}}},{{},{{{<!>},<!,!>!!!!a'>},<"o',!<a!>,<!!,!>},<a<!!!>!!!!,<>}}}},{{{{{{}}},{},{{<">},{{<!!!!}!><o!!!>!>,<>},{}},{{{<u''!>!>!>},<!!!>ui!"o!{!!"i!!!>>},{{}},{{{},{{<!!<!>"!!o'!u!!!>!!!>e}u!o>}}},{}}},{{},{<u!!a!>},<}iu}!{'!>},<!u'>}},{<i!>a}{}}<a!!!>!>!!!>{>}}}},{},{{{},{<"<!'!!a!>,<a{u!!!>!i>}},<!>,<iu!>!>!!a'<'!>},<!'!!">}}}},{{{<u"!!!>},<!>},<!!i!!<!>,<!!!>!>,<!!i!!!!<'!>},<>}},{<!>!>,<<!!>,<}e<!!!!ei!,,o"!!u!>,<!>,<"e!>,<u>},{<<o!>"<!!u!!!!!>},<{oe!>,<<>,<!!!!!>},<"!"{!>,<}!>},<u!!<u!>},<!!!>!>},<!!i{!>,<oe>}},{{}},{{{{{<!oe!!!>a!>},<<,>},{}},{}},{{},{{{},{<'!!ao!!!!a>}},{},{{<!>,<>}}},{<!!!>},<!!'{a!!}{o!}i!>},<{!!!>},<"!>},<u,>,<e!!!>o!o!!!!,!>,<!!'eu!!!!!>},<!>,<a}>}}},{<!!"!{!i!>},<au!<ua!>>,<!>!>!!!>'o!!o!!u'!,{>},{<<!'a!!!!!>i!>},<'!!e>,{<>}}}},{{<!!!>,<!!!!!!!>,<!>!>,<{!>,<!ue!!<!}'!!!!a!>o>},{{<!<!!!>a",!>!!!>!!!>'""o,!<}!!!>e>}}}},{{<!,!!!i,<i}}!>},<',!!!<!!>,<{!,"!!'o>},{{<!!!>u>}},{{},{<i!>!!'!!"<>}}}}},{{{{},{{{<}!>},<!!!>"!!e!>},<o}!>,<e>}}}},{{{<!>},<{ua!!!>uu!"!ii!!!!a!!!>!>o!>},<>},{}},{{{{<!!!!!!!>!>!>},<<!>,<!>},<e>}}}},{{{<>}},{<uu{{a!!!>!!"ueo!!e!!!!!!u'>}}},{{{<!!!!!!!>>},<<>},{<e}>}},{{{{{<o<'o,!>!>},<<!>!>},<<!>a!!,o>},<i<},,!>,<a!!ia!'ea>},{<!>,<u!>,<!>,<">}}},{{<<a!>,<}!>,!!!>!>,<<!>{>,<'<}!!}!>,<}!!e!>!!'!!}!!!!>},{},{{{{{<e!!!>!>'!>!!!>''!>},<!>},<!!"!u{!!!!o!!"!>!>>},<{!ou!!!>},<!>}'>},<!'!,"!!o!>,<">},{{{},{}},<o!>!!e!!",i}}!!{!!!>}<uo>},{{{<!!'">}},{{<'<!!!>,<!!!>}{<!!!>!e"!!!">},<!!!>},<!>i!>{!>},<"!!!>}!!i>}}}}}}},{{{{<{aeu!!,a!!!!a!>!{oi!!>}},{{{<>},{}}}},{{<!ia!>,<>,{<"'>,{<!,ie>,{<!!!>!!{eu!>,<,ea<>}}}},{{<!>,<!>,<!>},<"{o!!{!e'oeu<a},eou>}},{<,!>e>,<!>},<'!i'<e<ii}<!!>}},{{{<"<!>},<!}<!!"o,!>,<!i{iuea'!e>}},{},{{},{{{{{}},{<!>'""{u<!!o'!!!!!>!!!>!!!>'!!!!!>},<u!>o>}}},{{}}},{{{{{},{{{{<!>!!!!e>}}},{{<!!!>!!!!{}!!!>},<}>}}},{{<!!!!""!>},<!>},<}{!!oua}!>},<!!!euu}!>,<>},<!>,<!!!!!>ue!!ai!>},<{!>!>},<{'i,!a!!!!!{!!!>!>>}},{<"<!>},<i!!<u!!"a>},{{{{<!!!>u!!ee!"}"<!!!>!>,<!!!>"!>{!>,<!>},<a!!'>}},{{{<!>,<}{>},{<"a'ea!>}'{'!a!>,<!!'"!>,<!!!>>}}}},{<!!!!"e{"<!!!!{!!!!e!>,<,!><!!!>,<>,{}}}},{{{<'!o!!o!!{!>},<>}},{<i!!!!a"<>,<!>},<i!!}}>}},{{<!!{!!!><>,<e"!>},<<!>},<>},{<>,<!>},<<!>!!!>,<!!!>,<>},{<a!><"},!>},<>,{}}}},{{{<!>e"!!"!!ua!!!>>},{<a!!{'!!ei!>},<au>}},{<o!!!!e!><e!>},<{{o<!!!!">}},{{{{{{<i!!!>{'!>!!!!,a,i,,}!>,<!>},<{!>!>,<>},{<e!!!>o>}},{{<u!!!>!><"ioo!!'u}!!!>},<!<}>}}},{{<{e!!!!!><!>,<e!!!>""u!!eui>},{{{{{<!"!"!!e}"!!'!!o!!aaoa!>!>},<>}}},<ao!>,<!>},<!!"{iui!>!!!!,!!!><,!>,<a{u>}},{{<o!>},<!!i!u'ui!!!!!>!!,,!"!>,<}'!!!>!!>}}}}},{{{{<,!>,<!!!>},<!!!>},<e!>},<!!!>!>,<i!>},<!u{ee>},{<!'!>}>}},{{{<u<!>!!i!>{i!>,<>},<!!{!'!!}ao!>,<!>,<}!>},<!!!<>}}}}},{{{},{<!!!u!!!>!!!o"!!!>},<}!!'u!>,<>}},{{},{{<{{<}{!!!>!>e<!,i<{<<!>},<}>},{{<o>}}}}}},{{{<'!!!>!!'!!!>},<}{u>,{{{{<o}!iu{"}!!!>{!>,<!>,<!!!!'!!o!!!>a!>},<!>,<>}},{<!}i"o!>},<{!>,<<!>},<!>},<,!>,<,>}},<u'ei!!u!}>}},{{<!!e!>,<>},{{<!"ue!>i!>,<e>},{<}!oa!!"<{oi!>},<i,!>},<!}!!!>},<i!!>}}},{{},{},{}}},{{},{{{<!!iai!>},<!!!>,!{i<>,{}}},{{{<!!i,!!!!!!!>ei,!>},<!><>}}},{{{}},{{<a!!e!!!!'!>},<,}>,<'}!>},<!!!>!!!>a}!!!!!!<<""iu>},{{<>}},{{{}},{{<!!a!>,<'!><o!>,<o!!}a!,e!>,o!>,<'!!!>e>}}}},{<a<o},a!>e>,<!>!!!>>}}},{{{{}}},{}},{{<!!a"!!!!i!!!>!!!>!',>,<">},{{}}}}}},{{{<}'}!!}ou!!!a!>,<<a}}!!!>u!>i!!!>o>},<">},{{<!!!>!!}ae!!ei!!!>a!,ue,!>,<!>},<<>},{<!>,<eo!>!>!>!a!!}<!>'a<<i!>,<i!!!>>}},{{<!!a!>},<{!>{!>,<<!>,<,!'{>,<!!!!!>!!{'u"!!!!!!!!!>!>},<'!!i<i!!!>!!,!!!>>}}}}},{{{<{!>},<,!}!o!!!>,<!!!>!>u!>,<!!,!!i!!o"{>,<!>,<eu"<!>},<o!i<!>},<!"!!!!>},{{<!!!>ou{{aa>},<!>},<{'!!!>"!,}<!!!!'!!'>},{{<!!!>!'!!!<'!!!>},<"u"i>}}},{{},{{<!>!>!>,<u!{eu<e!>},<<'ii!>'!>},<>}},{{{{{{<>}}},<e!>{>},{<'<e!!!!!>,<!>,<}i"a<>}},{<!>,!!!!},,!>{!>,<!'""e>,<!>},<}"}!>},<!a!u}'o>}}},{{{<,"u!>},<'{>}},{}},{{{{<<'!>},<!!!{!>e!!{o"!>},<!>e!!a!"a!!!>o>},{<ue!>,<!>,<a!>'!!<,!>},<>}},{{{<!>},<{!!!!!>,u!!!>},<"!!!i!>!!!>,<>}},{{<!!,'!!ie!>o'!>,u}!!!>!>},<'!!,,o!!>},{{{{{{<!>{!!"u!>},<a}o!!<!>,<a!!"e>}}},{}},{<",{i>}}}}}}}},{{{{<!}a!!!>}<!>,<"!!<'!!!>!>},<i,'',u,>},<"!>e}!>a'>},{{{<,!>au"iue!>},<e}ai>},{}}},{{<!>},<{>},<ei"uua!>,<a}!!!>e,oi>}}}},{{{{<!>,<!!!>'!>,<"}{{<oa!!},u{'>}},{{<}!!!ue""{>,{{<'eoe!!!>>},{{<!a>}}}},{{<{!>,<{>}},{{<'''<i!{,!>,<,e!>},<o!!!>!!!>!><,!i>},{<e>,<"<!!!>!>,<eu!>!>!!!>e!>!><!!!>i>}}}},{{},{{{{{<<,o!!a'!!a<"!>,<{e}>,{<}>}}}},{{<i{>},<i}!o{!}',>},{{<"<!>a!oo!'!!<!!ea<!>,<>}}},{{},{{{<!>,<i}!!!>,<!!u{!!!!!>"!!!>e,"e!>},<!>},<!!o>},{<o>}},{{<!>,!!e!!'!>},<}<a<<>},{<o!!e!<!!!>'ei!>},<!!{!!!!!>>}},{{{{}},{<!!{oi!>!!e"o>}}}},{{{<a!>!!!!,ou!i'!!!>uu!>,<,>}},<!!!>"<!>},<o<!!!'a<u!!!>!!oi}!>,<o>}},{{{{<!!!>u{!!u}!>},<!>},<a!!!>e<!!ai}iea!!!>>},<!i,ia!!o"u!!!>a!!,}!>,<!!!>>},{<>,{{<'!>},<a!>,<o{!>,<!>,<i!uu>},{{<aiai!e'io}>},<!>,<{i!>,<'}!!}i!!!!!u!'!>},<!!!>o>}}},{{<,ii>},{{<!>},<!>,<,'!!'>,{<!!!>!!!'!>},<!}{!>a"!>},<eo!>},<!}<o<!o!!!!!!>}},{{<,}"!>!!!>!!!>,<"a!>},<!!!>!"ueu!>>},<!!!!>},{<u!>,<!>'a!!!!i'>}},{{{<!!!>!>,<!!'<eu!>,<}!!!!!>o>}}}}}},{{{{<!>},<!}!!u,{,<'ie<a"!'!>"">,{<!>,<,!>},<'oe!>!!!>,>}},{{<>,<!au,o>}},{<!u{>}},{{<e!>iuu{o!!{'>}},{{<<euu!>,<>},{{<!!e"u>}}}},{<<>,<<>},{<{>}}},{{<!!o}o<>,{}},{{<}>,{<!!{a!!!>o!>},<e!>},<iu!!e!>,<}!>},<'>,<u!>,<!!!>a"!!!!!>ii{{!>},<!>},<"",>}},{{{{},{},{{<!u!!!>}a{!!'i!!!>!!!!!!{!!!>>}}},{{<!>,<{o!!!>u<!!{!>ia!>,<o"!>},<>,{<o!!!>}<!!!>},<"ee!ou'!o{e,uei!!>}}}}}},{<o,,!>,<!!!<'!>e!}!>>,{<!!!>"a!!>,{<{!!,{!>"!!,oiu}{{o!!i>}}}},{{{{{<!>},<a<!>u},"a!,>}},{}},{{<!>},<eaua{o!!i>,{}},<i!>},<!!!!"iu!!>}},{{<e'e}i<!>,<>,<e,u!!!!!<!!e"!!u!!!>,<!>,<io!!!>>},{{<!!au"!>},<!!!>!>}!!}<!!'<!!a>},{{<!!!>,<>},<">}},{<!>,<<a!>,<!au!!!><eu{,!>,<!>},<!!!>}!!!>}!>,<>}},{{{<{,ui!!!!!>},<!!u!>,<!!!>>},{<i!!!!!>!!!>a!!!>!!"},}!>,<<!!!>u"!>,<!>},<>}},{{<a}!>!"!>e!!!>},<}!!!>},<o<ao'>}}},{{<>},{{{<uu{o!>,<i<>}}},{{<iue!>,<!>},<'!!!>!>},<i!!>},{<u!!!>i!>},<!>,<!!!!!>!!!>!!"i<!!!>},<u"<o!<>}}}}}}}
)";

void parse(const std::string &str,
           std::function<void()> start_group,
           std::function<void()> end_group,
           std::function<void()> garbage_char)
{
    int index = 0;
    while (index < str.size())
    {
        switch (str[index])
        {
        case '{':
            start_group();
            index += 1;
            break;

        case '}':
            end_group();
            index += 1;
            break;

        case '<':
            // Garbage - skip until end
            index += 1;
            while (str[index] != '>')
            {
                if (str[index] == '!')
                    index += 2; // skip
                else
                {
                    index += 1;
                    garbage_char();
                }
            }
            index += 1;
            break;

        default:
            // Ignore
            index += 1;
            break;
        }
    }
}

int count_groups(const std::string &str)
{
    int result = 0;

    parse(
        str,
        [&]() { result += 1; },
        []() {},
        []() {});

    return result;
}

int total_score(const std::string &str)
{
    int result = 0;
    int level = 0;

    parse(
        str,
        [&]() { level += 1; result += level; },
        [&]() { level -= 1; },
        []() {});

    return result;
}

int garbage_count(const std::string &str)
{
    int result = 0;

    parse(
        str,
        []() { },
        []() { },
        [&]() { result += 1; });

    return result;
}

int main(int /*argc*/, const char */*argv*/[])
{
    assert(count_groups("{}") == 1);
    assert(count_groups("{{{}}}") == 3);
    assert(count_groups("{{},{}}") == 3);
    assert(count_groups("{{{},{},{{}}}}") == 6);
    assert(count_groups("{<{},{},{{}}>}") == 1);
    assert(count_groups("{<a>,<a>,<a>,<a>}") == 1);
    assert(count_groups("{{<a>},{<a>},{<a>},{<a>}}") == 5);
    assert(count_groups("{{<!>},{<!>},{<!>},{<a>}}") == 2);

    assert(total_score("{}") == 1);
    assert(total_score("{{{}}}") == 6);
    assert(total_score("{{},{}}") == 5);
    assert(total_score("{{{},{},{{}}}}") == 16);
    assert(total_score("{<a>,<a>,<a>,<a>}") == 1);
    assert(total_score("{{<ab>},{<ab>},{<ab>},{<ab>}}") == 9);
    assert(total_score("{{<!!>},{<!!>},{<!!>},{<!!>}") == 9);
    assert(total_score("{{<a!>},{<a!>},{<a!>},{<ab>}}") == 3);

    assert(garbage_count("<>") == 0);
    assert(garbage_count("<random characters>") == 17);
    assert(garbage_count("<<<<>") == 3);
    assert(garbage_count("<{!>}>") == 2);
    assert(garbage_count("<!!>") == 0);
    assert(garbage_count("<!!!>>") == 0);
    assert(garbage_count(R"(<{o"i!a,<{i<a>)") == 10);

    std::cout << "Answer #1: " << total_score(PUZZLE_INPUT) << std::endl;
    std::cout << "Answer #2: " << garbage_count(PUZZLE_INPUT) << std::endl;

    return 0;
}