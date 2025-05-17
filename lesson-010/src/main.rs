use leptos::prelude::*;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Student {
    pub first_name: String,
    pub last_name: String,
}

#[component]
pub fn ShowSimpleComponentList(students: Vec<Student>) -> impl IntoView {
    view!{
            <For
                each=move || students.clone()
                key=|state| state.clone()
                let:child>
                <div>First Name: {child.first_name}, Last Name: {child.last_name} </div>
            </For>
    }
}

#[component]
pub fn ShowSimpleComponentListClone(students: Vec<Student>) -> impl IntoView {
    view!{
            <For
                each=move || students.clone()
                key=|state| state.clone()
                let:child>
                {   // Sometimes you need to clone in intermediate
                    let first_name = child.first_name.clone();
                    let last_name = child.last_name.clone();
                    view! {
                        <div>First Name: {first_name}, Last Name: {last_name} </div>
                    }
                }
            </For>
    }
}

#[component]
pub fn ShowSubComponentList(students: Vec<Student>) -> impl IntoView {
    view!{
            <For
                each=move || students.clone()
                key=|state| state.clone()
                let:child>
                <SubComponent student = child />
            </For>
    }
}

#[component]
pub fn SubComponent(student: Student) -> impl IntoView {
    view!{
        <div>first name: {student.clone().first_name}, last name: {student.clone().last_name} </div>
    }
}

fn main() {
    let student_list = vec![
        Student {first_name: "Tom".to_string(), last_name: "Boy".to_string()},
        Student {first_name: "Clara".to_string(), last_name: "Girl".to_string()},
        Student {first_name: "Sunny".to_string(), last_name: "Dog".to_string()},
        Student {first_name: "Happy".to_string(), last_name: "Cat".to_string()},
    ];
    
    mount_to_body(move || view! { 
        <div>
            <p>Single Component</p>
            <ShowSimpleComponentList students = student_list.clone()/>
        </div>
        <br/><hr/><br/>
        <div>
            <p>Single Component Clone</p>
            <ShowSimpleComponentListClone students = student_list.clone()/>
        </div>
        <br/><hr/><br/>
        <div>
            <p>Sub Component</p>
            <ShowSubComponentList students = student_list.clone()/>
        </div>
    })
}