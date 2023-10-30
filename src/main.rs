fn base(n: u32) -> f32 {
    let n = n as f32;
    2.628 * n + 0.064 * n * (n - 1.0)
}
fn main() {
    let dagger = 12.0;
    let boots = 35.0;
    let stride = 2.0;

    let zeal_or_axe = 15.0;
    let spins = 25.0;

    let attack_speed_rune = 1.0;

    let spins_at_1 = 7.0;

    let mut last_values: Vec<f32> = vec![0.0; 11];
    let mut last_values_rune: Vec<f32> = vec![0.0; 11];
    for i in 0..18 {
        let base = base(i);
        println!("\n## {}\n", i+1);
        let i = i as f32;

        let only_dagger = base + dagger;
        let only_boots = base + boots;
        let only_stride = base + stride;
        let only_axe = base + zeal_or_axe;
        let standard = base + stride + boots;


        let map = vec![
            ("base stats", base),
            ("only dagger", only_dagger),
            ("only boots", only_boots),
            ("only stride", only_stride),
            ("hearthbound axe", only_axe),
            ("boots with dagger", only_boots + dagger),
            ("hearthbound axe + boots", only_axe + boots),
            ("Stridebreaker + boots", standard),
            ("Stridebreaker + boots + one zeal", standard + zeal_or_axe),
            ("Stridebreaker + boots + two zeals", standard + zeal_or_axe * 2.0),
            ("Stridebreaker + boots + one dagger", standard + dagger),
        ];

        let spin_map = map
            .iter()
            .map(|(name, speed)| (name, (speed / spins).floor() + spins_at_1))
            .collect::<Vec<_>>();

        for (n, (name, spins)) in spin_map.iter().enumerate() {
            if spins > &last_values[n] {
                println!("At level {}, {name} gives {spins} spins", i + 1.0);
                println!();
            }
        }
        last_values = spin_map.iter().map(|(_, speed)| *speed).collect::<Vec<_>>();

        let spin_map_rune = map
            .iter()
            .map(|(name, speed)| {
                (
                    name,
                    ((attack_speed_rune + speed) / spins).floor() + spins_at_1,
                )
            })
            .collect::<Vec<_>>();

        for (n, (name, spins)) in spin_map_rune.iter().enumerate() {
            // only print if it's better than the non-rune version
            if spins > &last_values_rune[n] && spins > &last_values[n]
            {
                println!("At level {}, with rune {name} gives {spins} spins", i + 1.0);
                println!();
            }
        }
        last_values_rune = spin_map_rune
            .iter()
            .map(|(_, speed)| *speed)
            .collect::<Vec<_>>();
    }
}
