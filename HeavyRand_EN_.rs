use std::io;
use std::io::Write;

mod rand;

fn main() {
    println!("=== ZERO-DEP CHUNKY ROULETTE 9000 ===");

    let specializations = [
        "MESH SHIELD ( NERF THIS PLS )",
        "WINCH CLAW ( GET OVER HERE!!! )",
        "GOO GUN ( MINECRAFT BUILDER MODE )",
        "CHARGE 'N' SLAM ( OH YEAH, KOOL-AID MAN )"
    ];

    let weapons = [
        "KS-23 ( SHOTGUN SNIPER )",
        "SHAK-50 ( DOUBLE TAP MACHINE )",
        "SLEDGEHAMMER ( SLEDGEHAMMER GO BRRR )",
        "SPEAR ( HELICOPTER HELICOPTER )",
        "LEWIS GUN ( CHAT GGPT: NO RECOIL )",
        "M60 ( NO RELOAD, JUST PRAY )",
        "MGL32 ( SPAM TO WIN )",
        "FLAMETHROWER ( W+M1 PYRO )",
        "M134 MINIGUN ( SAY HELLO TO MY LITTLE FRIEND )",
        "BFR TITAN ( SWEATY TRYHARD PIC )",
        "SA1216 ( ROOM CLEANER )",
        ".50 AKIMBO ( RIP MY WRISTS )"
    ];

    let mut gadgets = vec! [
        "RPG-7 ( LIGHT DESTROYER 3000 )",
        "PROXIMITY SENSOR ( I SEE YOU )",
        "DOME SHIELD ( PANIC BUBBLE )",
        "HEALING EMITTER ( NOT A POCKET MED )",
        "GOO GRENADE ( PORTABLE WALL )",
        "FRAG GRENADE ( YEET! )",
        "PYRO GRENADE ( SPICY COCKTAIL )",
        "SMOKE GRENADE ( WHERE AM I? )",
        "FLASHBANG ( MY EYES!!! )",
        "EXPLOSIVE MINE ( WATCH YOUR STEP )",
        "BARRICADE ( NO ENTRY )",
        "PYRO MINE ( SATAN'S DOORMAT )",
        "C4 ( NUKE DELIVERY SERVICE )",
        "LOCKBOLT ( KINKY RESTRAINTS )",
        "GAS GRENADE ( SMELLS LIKE VICTORY )",
        "ANTI-GRAVITY CUBE ( WE ARE FLYING, BOYS! )"
    ];

    let wait_for_click = |text: &str| {
        println!("{}", text);
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
    };

    println!("System is ready for build generation");

    // 1. spec gen
    wait_for_click("👉 Press [Enter] to roll your ABILITY...");
    let spec_idx = rand::gen_range(0, (specializations.len() - 1) as u32) as usize;
    let chosen_spec = specializations[spec_idx];
    println!(" 🛡️ABILITY: {}", chosen_spec);

    // 2. gun gen
    wait_for_click("👉 Press [Enter] to roll your WEAPON...");
    let weapon_idx = rand::gen_range(0, (weapons.len() - 1) as u32) as usize;
    let chosen_weapon = weapons[weapon_idx];
    println!(" 🔫WEAPON: {}", chosen_weapon);

    // 3. gadget gen
    wait_for_click("👉 Press [Enter] to roll your GADGETS...");

    let g1_idx = rand::gen_range(0, (gadgets.len() - 1) as u32) as usize;
    let gadget_slot1 = gadgets.remove(g1_idx);

    let g2_idx = rand::gen_range(0, (gadgets.len() - 1) as u32) as usize;
    let gadget_slot2 = gadgets.remove(g2_idx);

    let g3_idx = rand::gen_range(0, (gadgets.len() - 1) as u32) as usize;
    let gadget_slot3 = gadgets.remove(g3_idx);

    println!(" 🎒 GADGET 1:  {}", gadget_slot1);
    println!(" 🎒 GADGET 2:  {}", gadget_slot2);
    println!(" 🎒 GADGET 3:  {}", gadget_slot3);

    println!("=== LOADOUT LOCKED IN! TIME TO SMASH! ===");

    // Program end
    println!("Press [Enter] to close the program...");
    io::stdout().flush().unwrap();
    let mut exit_buf = String::new();
    io::stdin().read_line(&mut exit_buf).unwrap();
}
