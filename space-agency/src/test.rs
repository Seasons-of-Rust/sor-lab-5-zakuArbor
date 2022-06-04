use crate::{calc_score, get_job_score, rank};
use personnel::{AstronautJob, Candidate};

#[test]
fn test_get_job_score() {
    assert_eq!(get_job_score(&AstronautJob::Biogeochemist), 251);
    assert_eq!(get_job_score(&AstronautJob::Biologist), 257);
    assert_eq!(get_job_score(&AstronautJob::Engineer), 263);
    assert_eq!(get_job_score(&AstronautJob::Geologist), 269);
    assert_eq!(get_job_score(&AstronautJob::Mechanic), 271);
    assert_eq!(get_job_score(&AstronautJob::Medic), 277);
    assert_eq!(get_job_score(&AstronautJob::RoverOp), 281);
    assert_eq!(get_job_score(&AstronautJob::Scientist), 283);
}

#[test]
fn test_calc_score() {
    assert_eq!(
        calc_score(&Candidate {
            primary_job: AstronautJob::Scientist,
            secondary_job: None,
            age: 31,
            health: 60
        }),
        2635
    );
    assert_eq!(
        calc_score(&Candidate {
            primary_job: AstronautJob::Scientist,
            secondary_job: Some(AstronautJob::Scientist),
            age: 31,
            health: 60
        }),
        2635
    ); //should be the same as the previous case
    assert_eq!(
        calc_score(&Candidate {
            primary_job: AstronautJob::Medic,
            secondary_job: Some(AstronautJob::Biologist),
            age: 25,
            health: 53
        }),
        1994
    );
    assert_eq!(
        calc_score(&Candidate {
            primary_job: AstronautJob::Scientist,
            secondary_job: None,
            age: 0,
            health: 30
        }),
        0
    ); // an non-existent human (should be not possible usually)
}

#[test]
fn test_rank() {
    let candidates_exp = vec![
        Candidate {
            primary_job: AstronautJob::Scientist,
            secondary_job: None,
            age: 31,
            health: 60,
        },
        Candidate {
            primary_job: AstronautJob::Medic,
            secondary_job: Some(AstronautJob::Biologist),
            age: 25,
            health: 53,
        },
        Candidate {
            primary_job: AstronautJob::Scientist,
            secondary_job: None,
            age: 0,
            health: 30,
        },
    ];
    let mut candidates = vec![
        candidates_exp[0].clone(),
        candidates_exp[2].clone(),
        candidates_exp[1].clone(),
    ];

    rank(&mut candidates);
    verify(&candidates_exp, &candidates, 1);

    let mut candidates = vec![
        candidates_exp[0].clone(),
        candidates_exp[2].clone(),
        candidates_exp[1].clone(),
    ];
    rank(&mut candidates);
    verify(&candidates_exp, &candidates, 2);

    let mut candidates = vec![
        candidates_exp[1].clone(),
        candidates_exp[0].clone(),
        candidates_exp[2].clone(),
    ];
    rank(&mut candidates);
    verify(&candidates_exp, &candidates, 3);

    let mut candidates = vec![
        candidates_exp[1].clone(),
        candidates_exp[2].clone(),
        candidates_exp[0].clone(),
    ];
    rank(&mut candidates);
    verify(&candidates_exp, &candidates, 4);

    let mut candidates = vec![
        candidates_exp[2].clone(),
        candidates_exp[0].clone(),
        candidates_exp[1].clone(),
    ];
    rank(&mut candidates);
    verify(&candidates_exp, &candidates, 5);

    let mut candidates = vec![
        candidates_exp[2].clone(),
        candidates_exp[1].clone(),
        candidates_exp[0].clone(),
    ];
    rank(&mut candidates);
    verify(&candidates_exp, &candidates, 6);
}

fn verify(candidates_exp: &Vec<Candidate>, candidates: &Vec<Candidate>, i: u8) {
    assert!(
        candidates_exp.len() == candidates.len(),
        "permutation {}: length are equal",
        i
    );
    assert!(
        candidates_exp[0] == candidates[0],
        "permutation {}: first element are equal",
        i
    );
    assert!(
        candidates_exp[1] == candidates[1],
        "permutation {}: first element are equal",
        i
    );
    assert!(
        candidates_exp[2] == candidates[2],
        "permutation {}: first element are equal",
        i
    );
}
