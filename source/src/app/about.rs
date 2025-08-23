use super::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Card title="About Me".to_string()>
            <div class="text-justify indent-6 text-base-content/70">
                "After high school, I initially chose to study industrial engineering but
                opted for economics at the University of Cologne. However, driven by my
                passion for technology, I decided to pursue engineering as a career. I
                completed my Bachelor's in mechanical engineering at the Technical
                University of Braunschweig, followed by a Master's degree. Throughout my
                studies, I focused on methodological depth, eventually specializing in
                mechatronics. This allowed me to combine traditional engineering with my
                passion for programming, giving me a broader perspective and enabling me
                to move beyond siloed thinking."
            </div>
        </Card>
    }
}
