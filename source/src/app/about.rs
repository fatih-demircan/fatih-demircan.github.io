use super::*;

#[component]
fn Paragraph(children: Children) -> impl IntoView {
    view! { <div class="text-justify indent-6 sm:indent-12">{children()}</div> }
}

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Card title="About".to_string()>
            <Paragraph>
                "I'm a mechanical engineer - who also holds a degree in economics - with
                a passion for programming."
            </Paragraph>
            <Paragraph>
                "After high school, I initially chose to study industrial engineering but
                opted for economics at the University of Cologne. However, driven by my
                passion for technology, I decided to pursue engineering as a career. I
                completed my Bachelor's in mechanical engineering at the Technical
                University of Braunschweig, followed by a Master's degree. Throughout my
                studies, I focused on methodological depth, eventually specializing in
                mechatronics. This allowed me to combine traditional engineering with my
                passion for programming, giving me a broader perspective and enabling me
                to move beyond siloed thinking."
            </Paragraph>
            <Paragraph>
                "In case you have interesting projects or potential business inquiries,
                feel free to contact me."
            </Paragraph>
        </Card>
    }
}
