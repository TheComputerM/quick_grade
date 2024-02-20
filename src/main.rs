#![allow(non_snake_case)]

use dioxus::prelude::*;


fn main() {
    dioxus_desktop::launch(App);
}

struct CourseEntry {
    name: String,
    credit: u16,
    grade: u16,
}

struct Courses(Vec<CourseEntry>);

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Courses(Vec::new()));

    cx.render(rsx! {
        Form {}
        CourseList {}
        FinalSG {}
    })
}

fn Form(cx: Scope) -> Element {
    let courses = use_shared_state::<Courses>(cx).unwrap();
	
    cx.render(rsx! {
        form {
            onsubmit: move |event| {
                let name = event.data.values["name"][0].clone();
                let credit = event.data.values["credit"][0].parse::<u16>().unwrap();
                let grade = event.data.values["grade"][0].parse::<u16>().unwrap();
                let entry = CourseEntry {
                    name: name,
                    credit: credit,
                    grade: grade
                };
                courses.write().0.push(entry);
            },
            div {
                label {
                    r#for: "name",
                    "Enter course name"
                }
                input {
                    r#type: "text",
                    name: "name",
                },
            }
            div {
                label {
                    r#for: "credit",
                    "Select course credits"
                }
                select {
                    name: "credit",
                    for i in 1..6 {
                        option {
                            value: i,
                            format!("{i} credit course")
                        }
                    }
                },
            }
            div {
                label {
                    r#for: "grade",
                    "Select grade"
                }
                select {
                    name: "grade",
                    option { value: 10, "A" },
                    option { value: 9, "A-" },
                    option { value: 8, "B" },
                    option { value: 7, "B-" },
                    option { value: 6, "C" },
                    option { value: 5, "C-" },
                    option { value: 4, "D" },
                    option { value: 2, "E" },
                },
            }
            input { r#type: "submit", },
           
            input { r#type: "reset", value: "Reset"},
        }
    })
}

fn CourseList(cx: Scope) -> Element {
    let courses = use_shared_state::<Courses>(cx).unwrap();

    cx.render(rsx! {
        ul {
            for course in courses.read().0.iter() {
                li {
                    format!("Name: {}, Credit: {}, Grade: {}", course.name, course.credit, course.grade)
                }
            }
        }
    })
}

fn FinalSG(cx: Scope) -> Element {
    let courses = &use_shared_state::<Courses>(cx).unwrap().read().0;
    let total_credits = courses.iter().fold(0, |acc, course| acc + course.credit);
    let aquired_credits = courses.iter().fold(0, |acc, course| acc + course.credit * course.grade);
    let sg = f64::from(aquired_credits) / if total_credits == 0 {
        1.0
    } else {
        f64::from(total_credits)
    };
    cx.render(rsx! {
        p {
            sg.to_string()
        }
    })
}

