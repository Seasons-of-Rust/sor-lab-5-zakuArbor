use personnel::{AstronautJob, Candidate};

const MAX_JOB_SCORE: u32 = 576;
const MAX_SCORE: u32 = 3928;

fn main() {
    let mut candidates: Vec<Candidate> = Candidate::load_candidate_file();
    rank(&mut candidates);
}

fn rank(candidates: &mut [Candidate]) {
    candidates.sort_by(|a: &Candidate, b: &Candidate| calc_score(b).cmp(&calc_score(a)));
}

fn calc_score(candidate: &Candidate) -> u32 {
    let mut score: u32 = get_job_score(&candidate.primary_job);

    // if let Some(job) =
    match &candidate.secondary_job {
        None => score *= score,
        Some(job) => score *= get_job_score(job),
    }
    score %= MAX_JOB_SCORE;
    score += candidate.health as u32;
    print!("{}", score);
    (score * candidate.age as u32) % MAX_SCORE
}

fn get_job_score(job: &AstronautJob) -> u32 {
    match job {
        AstronautJob::Biogeochemist => 251,
        AstronautJob::Biologist => 257,
        AstronautJob::Engineer => 263,
        AstronautJob::Geologist => 269,
        AstronautJob::Mechanic => 271,
        AstronautJob::Medic => 277,
        AstronautJob::RoverOp => 281,
        AstronautJob::Scientist => 283,
    }
}

#[cfg(test)]
mod test;
