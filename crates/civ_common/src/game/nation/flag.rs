use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Hash, Eq, Display, EnumString, EnumIter,
)]
pub enum Flag {
    Abkhazia,
    Aborigines,
    Acadia,
    Aceh,
    Acre,
    Adygea,
    Afghanistan,
    Africa,
    Ainu,
    Akwe,
    Aland,
    Alaska,
    Albania,
    Aleut,
    Algeria,
    Almohad,
    Alsace,
    Amazigh,
    Amazon,
    Andorra,
    Angola,
    Animals,
    Anhalt,
    Anishinaabe,
    Antarctica,
    AntarcticaAlt,
    AntiguaAndBarbuda,
    Apache,
    Arab,
    Aragon,
    Aram,
    Argentina,
    Armenia,
    Ashanti,
    Assam,
    Assyria,
    Asturias,
    Atlantis,
    Australia,
    Austria,
    Avar,
    Aymara,
    Azerbaijan,
    Aztec,
    Babylon,
    Baden,
    Bahamas,
    Bahrain,
    Bangladesh,
    Barbados,
    Barbarian,
    Bashkortostan,
    Bavarian,
    Belarus,
    Belgic,
    Belgium,
    Belize,
    Bengal,
    Benin,
    BeninAncient,
    Bhutan,
    Biafra,
    Boer,
    Boii,
    Bolivia,
    Bophuthatswana,
    Bosnia,
    Bosporus,
    Botswana,
    Brandenburg,
    Brasil,
    Britannia,
    Brittany,
    Brunei,
    Bulgaria,
    Burgundic,
    Burgundy,
    BurkinaFaso,
    Burundi,
    Buryatia,
    Byzantium,
    Caddo,
    California,
    Cameroon,
    Canada,
    CanadaOld,
    Canar,
    Cantonese,
    CapeVerde,
    Car,
    Carantanian,
    Cartago,
    Castile,
    Catalan,
    Celtiberian,
    Celtic,
    CentralAmerica,
    CentralLithuania,
    Chad,
    Cham,
    Chananea,
    Chechnya,
    Cheyenne,
    Chiapas,
    Chickasaw,
    Chile,
    Chimu,
    China,
    Choctaw,
    Chola,
    Chrobatian,
    Chumash,
    Chuvashia,
    Clatsop,
    Colombia,
    Comanche,
    Comoros,
    Conch,
    Constantine,
    Cornwall,
    Corsica,
    Cossack,
    CostaRica,
    Courland,
    Crete,
    CrimeanTatar,
    Croatia,
    Cuba,
    Cuyavia,
    Cyprus,
    Cyrenaica,
    Czech,
    Czechoslovakia,
    Dacian,
    Dagestan,
    Dahomey,
    Darfur,
    Ddr,
    Denmark,
    Dgb,
    Djibouti,
    Dominica,
    DominicanRepublic,
    Donetsk,
    DrCongo,
    Dryad,
    Ecuador,
    EastTimor,
    EgyptAncient,
    Egypt,
    Elam,
    ElSalvador,
    England,
    Epirus,
    EquatorialGuinea,
    Esperanto,
    Estonia,
    Eritrea,
    Ethiopia,
    EthiopiaOld,
    Etruscan,
    Europe,
    Euskadi,
    Evenkia,
    Faroes,
    Fiji,
    Finland,
    Flanders,
    Florence,
    Florida,
    Formosan,
    FranceOld,
    France,
    Franconia,
    FrenchPolynesia,
    Frisia,
    Friuli,
    Gabon,
    Gael,
    Galicia,
    Gambia,
    Gaul,
    Georgia,
    Gepid,
    Germanic,
    Germany,
    Gokturk,
    Gothic,
    Ghana,
    GhanaAncient,
    Ghaznavid,
    GoldenHorde,
    GreaterPoland,
    GreeceAncient,
    Greece,
    Greenland,
    Grenada,
    Grisons,
    Guanche,
    Guarani,
    Guatemala,
    Guinea,
    GuineaBissau,
    Gupta,
    Guyana,
    Hacker,
    Hainan,
    Haiti,
    Han,
    Hanover,
    Hansa,
    Hawaii,
    Helvetii,
    Hephthalite,
    Hesse,
    Himyar,
    Hittite,
    Honduras,
    Hopi,
    Hre,
    Hungary,
    Hunnic,
    Iberian,
    Iceland,
    Illyria,
    Inca,
    India,
    Indoeuropean,
    Indonesia,
    Innu,
    IranAncient,
    Iran,
    IraqOld,
    Iraq,
    Ireland,
    Iroquois,
    Israel,
    IsraelAncient,
    ItalianGreek,
    Italy,
    IvoryCoast,
    Jaffna,
    Jamaica,
    Japan,
    Jbonai,
    Jerusalem,
    Jolof,
    Jordan,
    Kalmykia,
    Kampuchea,
    KanemBornu,
    Karelia,
    Karen,
    Kashmir,
    Kashubia,
    Katanga,
    Kazakhstan,
    Keetoowah,
    Kenya,
    Khazaria,
    Khmer,
    Khoisan,
    Khwarezm,
    Kiev,
    Kiribati,
    Komi,
    Kongo,
    Korea,
    KoreaAncient,
    Kosovo,
    KunaYala,
    Kurd,
    Kushan,
    Kuwait,
    Kyrgyzstan,
    Labarum,
    Lombardy,
    Laos,
    LatinEmpire,
    Latvia,
    Lebanon,
    Lendian,
    Leon,
    Lesotho,
    LesothoOld,
    Liberia,
    Liburnian,
    Libya,
    LibyaOld,
    Liechtenstein,
    Liguria,
    Ligurian,
    Lipkatatar,
    Lippe,
    Lithuania,
    Lorraine,
    Louisiana,
    Luhansk,
    Luik,
    Luna,
    Lusatia,
    Luwian,
    Luxembourg,
    Lycian,
    Maasai,
    Macedon,
    Macedonia,
    Madagascar,
    Majapahit,
    Malawi,
    Malaysia,
    Maldives,
    Mali,
    MaliAncient,
    Malta,
    Mamluk,
    Man,
    Manchuria,
    Maori,
    Mapuche,
    Marathi,
    Mars,
    MarshallIslands,
    Mauritania,
    Mauritius,
    Maya,
    Mazovia,
    Mecklenburg,
    Median,
    Messapian,
    Metis,
    Mexico,
    Micronesia,
    Mikmaq,
    Milan,
    Minnesota,
    Miskito,
    Mitanni,
    Mixtec,
    Moldova,
    Moluccas,
    Mon,
    MonacoAlternative,
    Mongolia,
    Montenegro,
    Moravia,
    Mordovia,
    Morocco,
    Moscow,
    Mozambique,
    Mughal,
    Muskogee,
    Mwiska,
    Myanmar,
    MyanmarOld,
    NagornoKarabakh,
    Namibia,
    Naples,
    Nato,
    Nauru,
    Navajo,
    Nenetsia,
    Nepal,
    Nestoria,
    Netherlands,
    NetherlandsAntilles,
    Newfoundland,
    Newzealand,
    NezPerce,
    Nicaragua,
    Niger,
    Nigeria,
    Northernireland,
    NorthKorea,
    Northumbria,
    Norway,
    Normandy,
    Novgorod,
    Nubia,
    Numidia,
    Nunavut,
    NuuChahNulth,
    Occitania,
    Ohlone,
    Oldenburg,
    Oman,
    Ossetia,
    Otomi,
    Ottoman,
    Oz,
    Paeonia,
    Pakistan,
    Palatinate,
    Palau,
    Palestine,
    Palmyra,
    Panama,
    PapuaNewguinea,
    Paraguay,
    Parthia,
    Pashtun,
    Pelasgian,
    Peru,
    Philippines,
    Phoenicia,
    Phrygian,
    Pict,
    Piedmont,
    Pirate,
    Piratini,
    Poland,
    Polynesian,
    Pomerania,
    Portugal,
    Poyais,
    Prusai,
    Prussia,
    PuertoRico,
    Purhepecha,
    Qatar,
    Qing,
    Quebec,
    RapaNui,
    Raramuri,
    RCongo,
    Rhineland,
    Rif,
    Romania,
    Rome,
    Rvn,
    Russia,
    Rusyn,
    Rwanda,
    Ryukyu,
    Sabinium,
    Sadr,
    SaintKittsAndNevis,
    SaintLucia,
    Saka,
    Sakha,
    Salish,
    Samnium,
    Samoa,
    Samogitia,
    SanMarino,
    SaoTomeAndPrincipe,
    Sapmi,
    Sardinia,
    Sarmatia,
    SaudiArabia,
    Savoy,
    Saxony,
    Scania,
    SchleswigHolstein,
    Scotland,
    Scottishgaelic,
    Scythia,
    Seleucid,
    Seljuk,
    Seminole,
    Senegal,
    Serbia,
    Seychelles,
    Shan,
    Shawnee,
    Sherpa,
    Siberia,
    Sicily,
    SierraLeone,
    Sikh,
    Sikkim,
    Silesia,
    Singapore,
    Sinhalese,
    Slavic,
    Slovakia,
    Slovenia,
    Sokoto,
    SolomonIslands,
    Somalia,
    Somaliland,
    Songhai,
    SouthAfrica,
    SouthernCross,
    SouthernSudan,
    SouthYemen,
    Soviet,
    Spain,
    Srilanka,
    Srivijaya,
    Stpatrick,
    Sudan,
    Suebian,
    Sumeria,
    Suriname,
    Svg,
    Swahili,
    Swaziland,
    Sweden,
    Switzerland,
    Syria,
    Taino,
    Tairona,
    Taiwan,
    Tajikistan,
    Tanganyika,
    TannuTuva,
    Tanzania,
    Tatarstan,
    Templar,
    TeutonicOrder,
    Texas,
    Thailand,
    Thrace,
    Thuringia,
    Tibet,
    Timur,
    Tocharian,
    Togo,
    Tokipona,
    Toltec,
    Tonga,
    Transnistria,
    Transylvania,
    Trebizond,
    TrinidadAndTobago,
    Trnc,
    Tuareg,
    Tunisia,
    Tupi,
    Turkey,
    Turkmenistan,
    Tuvalu,
    Tyrol,
    Uae,
    Uganda,
    Ukraine,
    Unasur,
    UnitedKingdom,
    UnitedNations,
    Unknown,
    Urartu,
    Uruguay,
    Usa,
    Uyghur,
    Uzbekistan,
    Valknut,
    Vampire,
    Vandal,
    Vanuatu,
    Vatican,
    Vedic,
    Veletian,
    Venda,
    Venetic,
    Venezuela,
    Venice,
    Vermont,
    Vietnam,
    Viking,
    Visigoth,
    Vistulan,
    Volapuk,
    VolgaBulgar,
    VolgaGerman,
    Wales,
    Wallonia,
    WestIndiesFederation,
    WestPapua,
    Westphalia,
    Wuerttemberg,
    Xhosa,
    Xiongnu,
    Yemen,
    Yucatan,
    Yugoslavia,
    Zambia,
    Zanzibar,
    Zapotec,
    Zhuang,
    Zimbabwe,
    Zulu,
}
