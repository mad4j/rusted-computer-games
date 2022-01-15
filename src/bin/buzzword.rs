
const DATA: [[&str; 13]; 3] = [
    [
        "ABILITY",
        "BASAL",
        "BEHAVIORAL",
        "CHILD-CENTERED",
        "DIFFERENTIATED",
        "DISCOVERY",
        "FLEXIBLE",
        "HETEROGENEOUS",
        "HOMOGENEOUS",
        "MANIPULATIVE",
        "MODULAR",
        "TAVISTOCK",
        "INDIVIDUALIZED",
    ],
    [
        "LEARNING",
        "EVALUATIVE",
        "OBJECTIVE",
        "COGNITIVE",
        "ENRICHMENT",
        "SCHEDULING",
        "HUMANISTIC",
        "INTEGRATED",
        "NON-GRADED",
        "TRAINING",
        "VERTICAL AGE",
        "MOTIVATIONAL",
        "CREATIVE",
    ],
    [
        "GROUPING",
        "MODIFICATION",
        "ACCOUNTABILITY",
        "PROCESS",
        "CORE CURRICULUM",
        "ALGORITHM",
        "PERFORMANCE",
        "REINFORCEMENT",
        "OPEN CLASSROOM",
        "RESOURCE",
        "STRUCTURE",
        "FACILITY",
        "ENVIRONMENT",
    ],
];

fn main() {
    
    println!(
        "{} {} {}",
        DATA[0][rand::random::<usize>() % DATA[0].len()],
        DATA[1][rand::random::<usize>() % DATA[1].len()],
        DATA[2][rand::random::<usize>() % DATA[2].len()],
    );
}
