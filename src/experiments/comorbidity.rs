use crate::frames;

pub fn comorbidity_of_anxiety_or_mood_disorder() {
    // TODO: This belongs in frames - the graphical element can be exposed here, however.
    
    let dataset = frames::mental_health::patient_mental_health_conditions();

    println!("{}", dataset);
    // ScatterPlot::builder()
    //     .data(&dataset)
    //     .x("ADHD Type")
    //     .y("BIPOLAR")
    //     .build()
    //     .plot()
}