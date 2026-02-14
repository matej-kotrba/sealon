pub struct MainMenuData {}
pub struct CampData {}
pub struct DungeonData {}
pub enum Screen {
    MainMenu(MainMenuData),
    Camp(CampData),
    Shop(ShopData),
    Dungeon(DungeonData),
}

pub struct Game {
    pub screen: Screen,
}
impl Game {
    pub fn new() -> Self {
        Game {
            screen: Screen::MainMenu(MainMenuData {}),
        }
    }
}
