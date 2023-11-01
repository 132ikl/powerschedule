# powerschedule

Putting together a multi-semester course schedule for university is extremely difficult. Powerschedule handles everything for you, and generates every single possible schedule. It keeps track of class pre-requisites and co-requisites, which classes are offered in each semester, and can tell you if a schedule fulfills degree requirements.

Powerschedule was primarily designed to work with MSU's academic calendar, but could be adapted to work for other universities as well. Additionally, Powerschedule uses MSU's course requisite/corequisite syntax.

![Example output from powerschedule](/demo.png)


## Usage

Disclaimer: I am not liable for any faulty logic in powerschedule or faulty data given to powerschedule causing you to be unable to schedule classes or graduate on time. Double check your schedule manually and verify with an academic advisor before using output from powerschedule.

Powerschedule sources data from three files: `input.csv`, `taken.txt`, and `config.toml`:

* `input.csv` contains data about all of the classes you want to take. Lines started with `#` are ignored. The columns are as follows:
  * `subject`: the subject of the course
  * `number`: the course number (numbers only, sorry honors students!)
  * `credits`: number of credits
  * `required`: whether the course must be present in a schedule
  * `groups`: which groups a class is apart of (see config.toml), `|`-delimited
  * `semesters`: which semesters/terms a course is offered in, with an optional `Even`/`Odd` suffix to denote a course being offered during even or odd years. (examples: Fall, Summer, SpringOdd, FallEven)
  * `requisites`: pre-requisites and co-requisites for a course. see examples or requirements.rs for syntax details
* `taken.txt`: a plain-text file with all of the courses you have taken already, one course per line (syntax: ABC 100)
* `config.toml`: allows you to change the behavior of powerschedule. see provided example for Computer Engineering.
  * `min_credits`: the minimum credits allowed per semester (eg. if you're a full-time student, use the minimum number of credits to be considered full time)
  * `max_credits`: the maximum credits allowed per semester
  * `semesters`: the number of future semesters to calculate
  * `starting_term`: the first term to generate a schedule for
  * `groups`: groups of courses which need to meet a certain minimum credit threshold for a schedule to be complete. for example, if you need 10 credits from a certain category of courses, add a group for the category and mark each course which counts towards that group in `input.csv`.
  * `show_incomplete`: whether to show course schedules which do not include required classes or do not fulfill group credit requirement

Powerschedule currently requires Rust nightly. Install it with `rustup toolchain install nightly`, and run powerschedule with `cargo +nightly run --release` (optionally, set nightly to your default toolchain with `rustup default nightly`).

Powerschedule will generate all possible schedules, sorted by total number of credits. If powerschedule cannot find any schedules which meet the constraints given (such as the minimum and maximum credits per semester), it will not output any schedules. It will also indicate whether each schedule is "complete", that is, whether each schedule includes all required classes and whether the minimum credit requirements for each group is met.

Powerschedule will also output an "Errors" schedule, indicating how many semesters or schedules were considered invalid and for what reason. You can use this to help troubleshoot why powerschedule may not be generating schedules. For example, if many schedules are thrown out because a course isn't available during a term, the number of errors for "Not available in term [Season] [Year]" will be very high. Also, consider reducing the numbers of schedules into the future that powerschedule has to generate.

If you need to generate a schedule for many semesters in the future, powerschedule might run out of memory before schedules can be generated, or it may take an excessively long time to generate schedules. You may need to generate only 3-4 semesters into the future, decide which schedule you like the best, add those classes to `taken.txt`, advance `starting_term`, and generate more schedules into the future.

## Notes for MSU students

You can use the included `getclasses.py` script to automatically populate the `input.csv` file with data from the Office of the Registrar's website. Create a file named `input.txt` with one class per line (syntax: ABC 100). Requires selenium to be installed (use a venv). **Cross-reference data from the Registrar's website with the SIS.** I have seen multiple classes have erroneous info on the Registrar's website (eg., CSE 410 is listed as being offered only in Fall on reg.msu.edu, but in Fall and Spring on the SIS).

Feel free to reach out to me directly if you have any MSU-specific questions.

## Notes on tweaking for your needs

My use case is only for Fall and Spring semesters. If you are planning on taking summer classes, or your university uses a different term naming scheme, see data::TermSeason.

Course subjects and numbers are stored separately, and course numbers are stored as u16. If you need to support course numbers with letters in them, you'll need to either use a dummy number in place of the letter (eg. ABC300H -> ABC3000) or modify class::Class to use a String instead of a u16 (PRs welcome).
