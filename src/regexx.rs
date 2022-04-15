use regress::Regex;

pub fn run() {
    let regex = Regex::with_flags(r".+?(?=geil)", r"gs").unwrap();
    let some_string = "Lorenz ist so geil".to_string();
    for curr_match in regex.find_iter(&some_string) {
        print!("{:?}", &some_string[curr_match.range()]);
    }
    let stundenplan = Regex::with_flags(
        r#"(<td class="list inline_header" colspan="5">)(?:.*?(?=\1))"#,
        "gs",
    )
    .unwrap();
    let str = r#"                <td class="list inline_header" colspan="5">5a Klasse 5a mus</td>
            </tr>
            <tr class='list even'>
                <td class="list" align="center">5</td>
                <td class="list" align="center"><s>Gr</s>?Sd</td>
                <td class="list" align="center">Mk/N</td>
                <td class="list" align="center"><s>HAlt</s>?H1</td>
                <td class="list" align="center">5 Schüler (Tennis, Gr) mitbetreuen</td>
            </tr>
            <tr class='list odd'>
                <td class="list" align="center">6</td>
                <td class="list" align="center"><s>Gr</s>?Sd</td>
                <td class="list" align="center">Mk/N</td>
                <td class="list" align="center"><s>HAlt</s>?H1</td>
                <td class="list" align="center">s.o.</td>
            </tr>
            <tr class='list even'>
                <td class="list inline_header" colspan="5">5b Klasse 5b mus</td>
            </tr>
            <tr class='list odd'>
                <td class="list" align="center">5</td>
                <td class="list" align="center"><s>Gr</s>?Sd</td>
                <td class="list" align="center">Mk/N</td>
                <td class="list" align="center"><s>HAlt</s>?H1</td>
                <td class="list" align="center">5 Schüler (Tennis, Gr) mitbetreuen</td>
            </tr>
            <tr class='list even'>
                <td class="list" align="center">6</td>
                <td class="list" align="center"><s>Gr</s>?Sd</td>
                <td class="list" align="center">Mk/N</td>
                <td class="list" align="center"><s>HAlt</s>?H1</td>
                <td class="list" align="center">s.o.</td>
            </tr>
            <tr class='list odd'>
                <td class="list inline_header" colspan="5">5c Klasse 5c nat</td>
            </tr>
            <tr class='list even'>
                <td class="list" align="center">5</td>
                <td class="list" align="center"><s>Gr</s>?Mm</td>
                <td class="list" align="center">Mk/N</td>
                <td class="list" align="center"><s>HAlt</s>?LAa+b</td>
                <td class="list" align="center">7 Schüler (TEnnis, Gr) mitbetreuen</td>
            </tr>
            <tr class='list odd'>
                <td class="list" align="center">6</td>
                <td class="list" align="center"><s>Gr</s>?Mm</td>
                <td class="list" align="center">Mk/N</td>
                <td class="list" align="center"><s>HAlt</s>?LAa+b</td>
                <td class="list" align="center">s.o.</td>
            </tr>
            <tr class='list even'>
                <td class="list" align="center">6</td>
                <td class="list" align="center"><s>Gr</s>?Sd</td>
                <td class="list" align="center">Mk/N</td>
                <td class="list" align="center"><s>HAlt</s>?H1</td>
                <td class="list" align="center">s.o.</td>
            </tr>
            <tr class='list odd'>
                <td class="list inline_header" colspan="5">5d Klasse 5d nat</td>
            </tr>
            <tr class='list even'>
                <td class="list" align="center">5</td>
                <td class="list" align="center"><s>Gr</s>?Sd</td>
                <td class="list" align="center">Mk/N</td>
                <td class="list" align="center"><s>HAlt</s>?H1</td>
                <td class="list" align="center">5 Schüler (Tennis, Gr) mitbetreuen</td>
            </tr>
            <tr class='list odd'>
                <td class="list" align="center">6</td>
                <td class="list" align="center"><s>Gr</s>?Sd</td>
                <td class="list" align="center">Mk/N</td>
                <td class="list" align="center"><s>HAlt</s>?H1</td>
                <td class="list" align="center">s.o.</td>
            </tr>
            <tr class='list even'>
                <td class="list inline_header" colspan="5">6a Klasse 6a mus</td>
            </tr>
            <tr class='list odd'>
                <td class="list" align="center">1</td>
                <td class="list" align="center">Hi</td>
                <td class="list" align="center">E_2</td>
                <td class="list" align="center"><s>K177</s>?K207</td>
                <td class="list" align="center">&nbsp;</td>
            </tr>
            <tr class='list even'>
                <td class="list" align="center">2</td>
                <td class="list" align="center">Hi</td>
                <td class="list" align="center">E_2i</td>
                <td class="list" align="center"><s>K177</s>?K207</td>
                <td class="list" align="center">&nbsp;</td>
            </tr>
            <tr class='list odd'>
                <td class="list" align="center" style="background-color: #FFFFFF">3</td>
                <td class="list" align="center" style="background-color: #FFFFFF"><s>Rs</s>?Bl</td>
                <td class="list" align="center" style="background-color: #FFFFFF">G</td>
                <td class="list" align="center" style="background-color: #FFFFFF">K212</td>
                <td class="list" align="center" style="background-color: #FFFFFF">TL-Kraft</td>
            </tr>
            <tr class='list even'>
                <td class="list inline_header" colspan="5">6b Klasse 6b mus</td>
            </tr>
            <tr class='list odd'>
                <td class="list" align="center" style="background-color: #FFFFFF">3</td>
                <td class="list" align="center" style="background-color: #FFFFFF"><s>Gr</s>?Ko</td>
                <td class="list" align="center" style="background-color: #FFFFFF">L_2</td>
                <td class="list" align="center" style="background-color: #FFFFFF">K211</td>
                <td class="list" align="center">&nbsp;</td>
            </tr>
            <tr class='list even'>
                <td class="list inline_header" colspan="5">6c Klasse 6c mus</td>
            </tr>
            <tr class='list odd'>
                <td class="list" align="center" style="background-color: #FFFFFF">5</td>
                <td class="list" align="center" style="background-color: #FFFFFF"><s>Br</s>?Ec</td>
                <td class="list" align="center" style="background-color: #FFFFFF"><s>NT</s>?M</td>
                <td class="list" align="center" style="background-color: #FFFFFF">K213</td>
                <td class="list" align="center">&nbsp;</td>
            </tr>
            <tr class='list even'>"#;
    for m in stundenplan.find_iter(str) {
        print!("{}\n\n\n", &str[m.range()]);
    }
}
