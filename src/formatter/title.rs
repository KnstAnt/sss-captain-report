use chrono::Datelike;

//
pub struct Title {
    title: String,
}
//
impl Title {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_owned(),
        }
    }
    //
    pub fn print(&self) -> String {
        let local = chrono::Local::now();
        let year = local.year();
        format!(
"<HTML>

<HEAD>

<TITLE>Титульный лист</TITLE>

</HEAD>

<BODY>

<P> <CENTER> BoardTrix </CENTER>

<HR SIZE=4 WIDTH=100% NOSHADE ALIGN=CENTER>

<TABLE WIDTH=100%>

</TABLE>

<BR> <BR> <BR> <BR> <BR><BR> <BR> <BR> <BR> <BR>

<P><FONT SIZE=6>

<CENTER><B> Тестовые расчеты </B> </FONT> </CENTER>

<CENTER><B> {} </B> </FONT> </CENTER>

<CENTER><B> Расчет прочности и остойчивости </B> </FONT> </CENTER>

<BR> <BR> <BR> <BR> <BR> <BR> <BR> <BR> <BR> <BR> <BR> 

<B> <div align=\"right\">\\_\\_\\_\\_\\_\\_\\_\\_\\_\\_\\_\\_\\_/_____________ </div> </B> </FONT> 

<BR> <BR>

<P> <CENTER> Санкт-Петербург <BR> {} </CENTER>

</BODY>

</HTML>

<div style=\"page-break-after: always;\"></div>

",
/*
"<HTML>

<HEAD>

<TITLE>Титульный лист</TITLE>

</HEAD>

<BODY>

<P> <CENTER> <img src=\"../assets/sa_lab.png\" width=\"200\" height=\"120\" alt=\"Эмблема\"> </CENTER> </P>

<P> <CENTER> BoardTrix </CENTER>

<HR SIZE=4 WIDTH=100% NOSHADE ALIGN=CENTER>

<TABLE WIDTH=100%>

</TABLE>

<BR> <BR> <BR> <BR> <BR><BR> <BR> <BR> <BR> <BR>

<P><FONT SIZE=6>

<CENTER><B> Тестовые расчеты </B> </FONT> </CENTER>

<CENTER><B> {} </B> </FONT> </CENTER>

<CENTER><B> Расчет прочности и остойчивости </B> </FONT> </CENTER>

<BR> <BR> <BR> <BR> <BR> <BR> <BR> <BR> <BR> <BR> <BR> 

<B> <div align=\"right\">\\_\\_\\_\\_\\_\\_\\_\\_\\_\\_\\_\\_\\_/_____________ </div> </B> </FONT> 

<BR> <BR>

<P> <CENTER> Санкт-Петербург <BR> {} </CENTER>

</BODY>

</HTML>

<div style=\"page-break-after: always;\"></div>

",
*/
            self.title, year
        )
    }
}
