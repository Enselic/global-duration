mod checkpoint;

use checkpoint::Checkpoint;
use lazy_static::lazy_static;
use std::fs::OpenOptions;
use std::sync::Mutex;
use std::io::Write;

#[derive(Clone)]
enum To {
    Stderr,
    File(String),
}

// The main point of this crate is to make it easy to measure the duration between
// two arbitrary code locations. Globals are generally evil, but in this case it is
// exactly what we want, since passing a struct around from the first place
// to the second place often would be prohibitively much work.
lazy_static! {
    static ref LAST_CHECKPOINT: Mutex<Option<Checkpoint>> = Mutex::from(None);
    static ref TO: Mutex<To> = Mutex::from(To::Stderr);
}

pub fn to_file(path: &str) {
    let mut to = TO.lock().unwrap();
    *to = To::File(String::from(path));
}

pub fn checkpoint(name: &str) {
    let output = update_checkpoint(name);
    if let Some(output) = output {
        print(&output);
    }
}

fn update_checkpoint(new_name: &str) -> Option<String> {
    let mut last_checkpoint = LAST_CHECKPOINT.lock().unwrap();

    let output = match &*last_checkpoint {
        // No ouput first checkpoint to minimize effects on duration measurements
        None => None,
        Some(checkpoint) => Some(format!(
            "Hitting '{}' after {:?} since hitting '{}'\n",
            new_name,
            checkpoint.instant.elapsed(),
            checkpoint.name
        )),
    };

    *last_checkpoint = Some(Checkpoint::new(new_name));

    output
}

fn print(output: &str) {
    let to;
    {
        // Hold lock for minimal amount of time for minimal risk of e.g. lock poisoning
        to = TO.lock().unwrap().clone();
    }

    match to {
        To::Stderr => {
            eprint!("{}", output);
        }
        To::File(path) => {
            let mut file = OpenOptions::new().create(true).append(true).open(&path).unwrap();
            if let Err(e) = write!(file, "{}", output) {
                eprint!("Error while writing to {}: {:?}", &path, e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::thread::sleep;
    use std::time::Duration;

    // TODO: Port this test to and write more tests using assert_cmd
    #[test]
    fn basic_to_file() {
        // Do the test
        to_file("/tmp/com.setofskills.global_duration.test-output.txt");
        checkpoint("checkpoint 1");
        sleep(Duration::from_millis(1000));
        checkpoint("checkpoint 2");
        sleep(Duration::from_millis(2000));
        checkpoint("checkpoint 3");

        // TODO: Assert result
        // let actual_contents = std::fs::read_to_string("/tmp/global-duration-to-file.test-output.txt").unwrap();
    }
}
