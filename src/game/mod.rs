struct CampData {}
struct ShopData {}
struct DungeonData {}

enum Screen {
    Camp(CampData),
    Shop(ShopData),
    Dungeon(DungeonData),
}

struct Game {
    screen: Screen,
}
