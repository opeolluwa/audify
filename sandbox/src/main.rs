use std::error::Error;
use ::{*, models::{gtts::GttsModel, tts_rs::TtsModel, parler::ParlerModel, msedge::MSEdgeModel, meta::MetaModel}};

fn main() -> Result<(), Box<dyn Error>>{
    // Create the NaturalTts using the Builder pattern
    let mut natural = NaturalTtsBuilder::default()
        .default_model(Model::Gtts)
        .gtts_model(GttsModel::default())
        .parler_model(ParlerModel::default())
        .msedge_model(MSEdgeModel::default())
        .meta_model(MetaModel::default())
        .tts_model(TtsModel::default())
        .build()?;

    // Use the pre-included function to say a message using the default_model.
    let _ = natural.say_auto("Hello, World!".to_string())?;
}
