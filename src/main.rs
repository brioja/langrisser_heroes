use std::collections::HashSet;


#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Faction {
    Dark,
    Meteor,
    Mythical,
    Princess,
    Yeless,
    Origins,
    Glory,
    Protag,
    Rebirth,
    Strategic,
    Empire,
    Time
}

struct Hero
{
    name : String,
    factions : HashSet<Faction>,
}

struct Boss
{
    name : String,
    factions : HashSet<Faction>,
}

fn build_heroes() -> Vec<Hero>
{
    let mut heroes = Vec::new();

    heroes.push(Hero {
        name: "Ainz".to_string(),
        factions : [Faction::Rebirth, Faction::Dark, Faction::Time].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Aka".to_string(),
        factions : [Faction::Glory, Faction::Origins, Faction::Mythical].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Albedo".to_string(),
        factions : [Faction::Dark, Faction::Time, Faction::Rebirth].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Alicia".to_string(),
        factions : [Faction::Protag, Faction::Time, Faction::Mythical].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Alpha".to_string(),
        factions : [Faction::Empire, Faction::Dark].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Altemueller".to_string(),
        factions : [Faction::Empire, Faction::Strategic, Faction::Dark].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Alustriel".to_string(),
        factions : [Faction::Glory, Faction::Strategic, Faction::Origins].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Angelina".to_string(),
        factions : [Faction::Princess, Faction::Meteor, Faction::Yeless].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Archon of the Moon".to_string(),
        factions : [Faction::Strategic, Faction::Yeless, Faction::Mythical].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Ares".to_string(),
        factions : [Faction::Protag, Faction::Empire, Faction::Rebirth].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Arianrhod".to_string(),
        factions : [Faction::Rebirth, Faction::Mythical, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Ashram".to_string(),
        factions : [Faction::Dark, Faction::Strategic, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Ashumere".to_string(),
        factions : [Faction::Origins, Faction::Princess, Faction::Mythical].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Autokrato".to_string(),
        factions : [Faction::Empire, Faction::Rebirth].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Azusa".to_string(),
        factions : [Faction::Origins, Faction::Strategic, Faction::Mythical].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Bernhardt".to_string(),
        factions : [Faction::Empire, Faction::Dark, Faction::Mythical].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Betty".to_string(),
        factions : [Faction::Glory, Faction::Empire, Faction::Dark].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Bozel".to_string(),
        factions : [Faction::Mythical, Faction::Dark].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Brenda".to_string(),
        factions : [Faction::Yeless, Faction::Meteor].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Carolian".to_string(),
        factions : [Faction::Glory, Faction::Empire, Faction::Strategic].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Cherie".to_string(),
        factions : [Faction::Glory, Faction::Princess, Faction::Meteor].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Christiane".to_string(),
        factions : [Faction::Rebirth, Faction::Empire, Faction::Princess].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Clarett".to_string(),
        factions : [Faction::Princess, Faction::Meteor, Faction::Yeless].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Clotaire".to_string(),
        factions : [Faction::Strategic, Faction::Empire].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Deedlit".to_string(),
        factions : [Faction::Princess, Faction::Origins, Faction::Time].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Dieharte".to_string(),
        factions : [Faction::Protag, Faction::Origins, Faction::Meteor].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Elma".to_string(),
        factions : [Faction::Protag, Faction::Glory, Faction::Rebirth].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Elwin".to_string(),
        factions : [Faction::Protag, Faction::Glory, Faction::Empire].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Emilia".to_string(),
        factions : [Faction::Empire, Faction::Princess].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Emperor Lovina".to_string(),
        factions : [Faction::Protag, Faction::Strategic, Faction::Rebirth].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Epsilon".to_string(),
        factions : [Faction::Meteor, Faction::Dark, Faction::Yeless].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Estelle".to_string(),
        factions : [Faction::Protag, Faction::Glory, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Evil God Kreuger".to_string(),
        factions : [Faction::Mythical, Faction::Dark].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Florentia".to_string(),
        factions : [Faction::Empire, Faction::Strategic, Faction::Rebirth].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Gerold and Layla".to_string(),
        factions : [Faction::Origins, Faction::Strategic].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Gintoki Sakata".to_string(),
        factions : [Faction::Protag, Faction::Yeless, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Gizarof".to_string(),
        factions : [Faction::Dark, Faction::Mythical].iter().cloned().collect(),
    });    

    heroes.push(Hero {
        name: "Grenier".to_string(),
        factions : [Faction::Glory].iter().cloned().collect(),
    });    

    heroes.push(Hero {
        name: "Gustaf".to_string(),
        factions : [Faction::Dark, Faction::Mythical, Faction::Rebirth].iter().cloned().collect(),
    });    

    heroes.push(Hero {
        name: "Helena".to_string(),
        factions : [Faction::Glory, Faction::Empire].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Hiei".to_string(),
        factions : [Faction::Origins, Faction::Meteor, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Hilda".to_string(),
        factions : [Faction::Empire, Faction::Strategic, Faction::Rebirth].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Himiko".to_string(),
        factions : [Faction::Meteor, Faction::Yeless, Faction::Time].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Ilucia".to_string(),
        factions : [Faction::Glory, Faction::Strategic].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Ironblood Commander".to_string(),
        factions : [Faction::Empire, Faction::Strategic, Faction::Yeless].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Joshua".to_string(),
        factions : [Faction::Time, Faction::Meteor, Faction::Princess].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Jugler".to_string(),
        factions : [Faction::Origins, Faction::Meteor, Faction::Mythical].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Julian".to_string(),
        factions : [Faction::Dark, Faction::Meteor, Faction::Rebirth].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Kagura".to_string(),
        factions : [Faction::Protag, Faction::Princess, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Kagura".to_string(),
        factions : [Faction::Protag, Faction::Princess, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Kaguya".to_string(),
        factions : [Faction::Princess, Faction::Strategic, Faction::Mythical].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Kertez".to_string(),
        factions : [Faction::Dark, Faction::Rebirth].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "King of the Red Moon".to_string(),
        factions : [Faction::Origins, Faction::Dark, Faction::Mythical].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Knight of Mystery".to_string(),
        factions : [Faction::Strategic, Faction::Yeless].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Kurama".to_string(),
        factions : [Faction::Strategic, Faction::Yeless, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Lambda".to_string(),
        factions : [Faction::Protag, Faction::Yeless, Faction::Mythical].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Lana".to_string(),
        factions : [Faction::Dark, Faction::Princess].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Landius".to_string(),
        factions : [Faction::Protag, Faction::Yeless].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Lanford".to_string(),
        factions : [Faction::Strategic, Faction::Yeless].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Ledin".to_string(),
        factions : [Faction::Protag, Faction::Glory].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Leon".to_string(),
        factions : [Faction::Empire, Faction::Strategic].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Leonhardt".to_string(),
        factions : [Faction::Origins, Faction::Empire, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Liana".to_string(),
        factions : [Faction::Protag, Faction::Glory, Faction::Princess].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Licorice".to_string(),
        factions : [Faction::Dark, Faction::Princess, Faction::Rebirth].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Light of Genesis".to_string(),
        factions : [Faction::Origins, Faction::Glory, Faction::Rebirth].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Listell".to_string(),
        factions : [Faction::Dark, Faction::Yeless].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Lucretia".to_string(),
        factions : [Faction::Empire, Faction::Princess, Faction::Rebirth].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Luna".to_string(),
        factions : [Faction::Origins, Faction::Princess, Faction::Strategic].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Maiya".to_string(),
        factions : [Faction::Empire, Faction::Rebirth].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Mariandel".to_string(),
        factions : [Faction::Princess, Faction::Yeless, Faction::Mythical].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Mariel".to_string(),
        factions : [Faction::Glory, Faction::Rebirth].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "McClaine".to_string(),
        factions : [Faction::Strategic, Faction::Meteor, Faction::Yeless].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Michel".to_string(),
        factions : [Faction::Glory, Faction::Rebirth].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Mu".to_string(),
        factions : [Faction::Yeless, Faction::Mythical].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Noemi".to_string(),
        factions : [Faction::Glory, Faction::Rebirth].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Oboro".to_string(),
        factions : [Faction::Origins, Faction::Yeless, Faction::Mythical].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Omega".to_string(),
        factions : [Faction::Meteor, Faction::Dark].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Patsyr".to_string(),
        factions : [Faction::Dark, Faction::Rebirth].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Rachel".to_string(),
        factions : [Faction::Protag, Faction::Yeless].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Rainforce".to_string(),
        factions : [Faction::Strategic, Faction::Yeless, Faction::Mythical].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Rean".to_string(),
        factions : [Faction::Protag, Faction::Mythical, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Renata".to_string(),
        factions : [Faction::Dark, Faction::Rebirth].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Renne".to_string(),
        factions : [Faction::Dark, Faction::Yeless, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Ricky".to_string(),
        factions : [Faction::Protag, Faction::Glory, Faction::Yeless].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Rostam".to_string(),
        factions : [Faction::Origins, Faction::Strategic, Faction::Meteor].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Rozalia".to_string(),
        factions : [Faction::Glory, Faction::Rebirth, Faction::Princess].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Rozenciel".to_string(),
        factions : [Faction::Empire, Faction::Mythical, Faction::Princess].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Ryo Sanada".to_string(),
        factions : [Faction::Protag, Faction::Yeless, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Sage of the Trees".to_string(),
        factions : [Faction::Yeless, Faction::Mythical].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Saintess of the Ark".to_string(),
        factions : [Faction::Protag, Faction::Princess, Faction::Mythical].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Sakura Shinguji".to_string(),
        factions : [Faction::Empire, Faction::Protag, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Selvaria".to_string(),
        factions : [Faction::Empire, Faction::Strategic, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Shalltear".to_string(),
        factions : [Faction::Dark, Faction::Meteor, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Shelfaniel".to_string(),
        factions : [Faction::Princess, Faction::Yeless].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Shilinka".to_string(),
        factions : [Faction::Meteor, Faction::Mythical].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Sigma".to_string(),
        factions : [Faction::Protag, Faction::Meteor, Faction::Yeless].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Sissy White".to_string(),
        factions : [Faction::Glory, Faction::Princess].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Sumire".to_string(),
        factions : [Faction::Empire, Faction::Princess, Faction::Time].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Suzette".to_string(),
        factions : [Faction::Rebirth, Faction::Meteor].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Tiaris".to_string(),
        factions : [Faction::Protag, Faction::Origins, Faction::Princess].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Toguro Brothers".to_string(),
        factions : [Faction::Dark, Faction::Empire, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Towa".to_string(),
        factions : [Faction::Strategic, Faction::Origins, Faction::Rebirth].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Transcendent".to_string(),
        factions : [Faction::Mythical, Faction::Dark, Faction::Yeless].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Tsubame".to_string(),
        factions : [Faction::Meteor, Faction::Rebirth].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Vincent".to_string(),
        factions : [Faction::Meteor, Faction::Empire, Faction::Dark].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Virash".to_string(),
        factions : [Faction::Yeless, Faction::Meteor].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Wandering Duelist".to_string(),
        factions : [Faction::Origins, Faction::Meteor, Faction::Yeless].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Wataru".to_string(),
        factions : [Faction::Protag, Faction::Yeless, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Wehttam".to_string(),
        factions : [Faction::Protag, Faction::Dark, Faction::Mythical].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Werner".to_string(),
        factions : [Faction::Glory, Faction::Meteor, Faction::Yeless].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Werner Dime".to_string(),
        factions : [Faction::Empire, Faction::Rebirth].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Wiler".to_string(),
        factions : [Faction::Strategic, Faction::Yeless].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Young Jessica".to_string(),
        factions : [Faction::Glory, Faction::Origins, Faction::Mythical].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Yulia".to_string(),
        factions : [Faction::Glory, Faction::Mythical, Faction::Princess].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Yusuke".to_string(),
        factions : [Faction::Protag, Faction::Mythical, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Zerida".to_string(),
        factions : [Faction::Dark, Faction::Meteor, Faction::Mythical].iter().cloned().collect(),
    });

    // SR Heroes
    
    heroes.push(Hero {
        name: "Alfred".to_string(),
        factions : [Faction::Meteor, Faction::Yeless].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Altina".to_string(),
        factions : [Faction::Empire, Faction::Meteor, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Angelica".to_string(),
        factions : [Faction::Glory, Faction::Time].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Chris".to_string(),
        factions : [Faction::Protag, Faction::Glory, Faction::Princess].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Egbert".to_string(),
        factions : [Faction::Empire, Faction::Dark].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Emerick".to_string(),
        factions : [Faction::Empire, Faction::Strategic].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Feraquea".to_string(),
        factions : [Faction::Dark, Faction::Meteor].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Freya".to_string(),
        factions : [Faction::Origins, Faction::Princess].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Hein".to_string(),
        factions : [Faction::Glory, Faction::Empire, Faction::Princess].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Imelda".to_string(),
        factions : [Faction::Empire, Faction::Strategic].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Iris".to_string(),
        factions : [Faction::Empire, Faction::Princess, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Kirikaze".to_string(),
        factions : [Faction::Origins, Faction::Meteor].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Klose".to_string(),
        factions : [Faction::Princess, Faction::Yeless, Faction::Time].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Kazuma".to_string(),
        factions : [Faction::Glory, Faction::Meteor, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Lance".to_string(),
        factions : [Faction::Empire, Faction::Strategic, Faction::Glory].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Liffany".to_string(),
        factions : [Faction::Origins, Faction::Princess].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Melania".to_string(),
        factions : [Faction::Glory, Faction::Princess, Faction::Yeless].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Narm".to_string(),
        factions : [Faction::Glory, Faction::Princess, Faction::Meteor].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Oliver".to_string(),
        factions : [Faction::Yeless, Faction::Meteor, Faction::Glory].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Olivier".to_string(),
        factions : [Faction::Empire, Faction::Strategic, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Parn".to_string(),
        factions : [Faction::Protag, Faction::Origins, Faction::Time].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Pirotess".to_string(),
        factions : [Faction::Meteor, Faction::Dark, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Serena".to_string(),
        factions : [Faction::Strategic, Faction::Yeless].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Shibaraku".to_string(),
        factions : [Faction::Yeless, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Silverwolf".to_string(),
        factions : [Faction::Origins, Faction::Meteor].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Sonya".to_string(),
        factions : [Faction::Empire, Faction::Dark, Faction::Princess].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Sophia".to_string(),
        factions : [Faction::Origins, Faction::Princess, Faction::Mythical].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Touma".to_string(),
        factions : [Faction::Strategic, Faction::Time].iter().cloned().collect(),
    });

    heroes.push(Hero {
        name: "Vargas".to_string(),
        factions : [Faction::Empire, Faction::Strategic].iter().cloned().collect(),
    });
    
    heroes.push(Hero {
        name: "Varna".to_string(),
        factions : [Faction::Empire, Faction::Dark].iter().cloned().collect(),
    });
    
    // heroes.push(Hero {
    //     name: "".to_string(),
    //     factions : [Faction::, Faction::, Faction::].iter().cloned().collect(),
    // });
    
    return heroes;
}

fn build_bosses() -> Vec<Boss>
{
    let mut bosses = Vec::new();
    // bosses.push(Boss {
    //     name: "Fenrir".to_string(),
    //     factions : [Faction::Dark, Faction::Mythical].iter().cloned().collect(),
    // });

    bosses.push(Boss {
        name: "Nidhogg".to_string(),
        factions : [Faction::Princess, Faction::Yeless].iter().cloned().collect(),
    });

    // bosses.push(Boss {
    //     name: "Phoenix".to_string(),
    //     factions : [Faction::Protag, Faction::Yeless, Faction::Strategic].iter().cloned().collect(),
    // });
    
    bosses.push(Boss {
        name: "Jorgmungandr".to_string(),
        factions : [Faction::Origins, Faction::Meteor].iter().cloned().collect(),
    });
    
    bosses.push(Boss {
        name: "Huginn & Munin".to_string(),
        factions : [Faction::Glory, Faction::Protag].iter().cloned().collect(),
    });

    bosses.push(Boss {
        name: "Sleipnir".to_string(),
        factions : [Faction::Rebirth, Faction::Strategic, Faction::Empire].iter().cloned().collect(),
    });   
    
    return bosses;
}

fn print_hero_boss_frequency() {
    let heroes = build_heroes();
    let bosses = build_bosses();

    for hero in heroes.iter()
    {
        let mut faction_count = 0;

        for boss in bosses.iter()
        {
            // Do the sets intersect?
            let intersection: HashSet<_> = hero.factions.intersection(&boss.factions).collect();
            if !intersection.is_empty()
            {
                faction_count = faction_count + 1;
            }
        }

        println!("{} - {} ", faction_count, hero.name);
    }
}

fn count_heroes_in_this_group(faction_group : HashSet<Faction>) -> u32
{
    let heroes = build_heroes();

    let mut hero_count = 0;
    
    for hero in heroes.iter()
    {
        // Do the sets intersect?
        let intersection: HashSet<_> = hero.factions.intersection(&faction_group).collect();
        if !intersection.is_empty()
        {
            hero_count = hero_count + 1;
        }
    }

    return hero_count;
}


fn print_faction_overlap_frequency() {

    let heroes = build_heroes();

    // Select two factions
    // Is hero in eiether faction?
    // Print number of people in either faction

    // Build list of factions

    let factions = vec![Faction::Dark, Faction::Meteor, Faction::Mythical, Faction::Princess, Faction::Yeless,
                        Faction::Origins, Faction::Glory, Faction::Protag, Faction::Rebirth, Faction::Strategic,
                        Faction::Empire, Faction::Time ];

    // All faction groups
    for i in 0..factions.len()
    {
        for j in i + 1..factions.len()
        {
            let mut faction_group =  HashSet::new();
            faction_group.insert(factions[i]);
            faction_group.insert(factions[j]);
           
            // Count heroes in these two groups
            let hero_count = count_heroes_in_this_group(faction_group);

            println!("{:?} {:?}-{:?}", hero_count, factions[i], factions[j]);

        }
    }
   
}


fn main() {
//     print_hero_boss_frequency();
    print_faction_overlap_frequency();
}
