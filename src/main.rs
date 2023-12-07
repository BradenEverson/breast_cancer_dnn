use std::error::Error;
use triton_grow::network::{input::Input, network::Network};

use breast_cancer::models::med_model::MedModel;

fn main() -> Result<(), Box<dyn Error>>{
    let mut models: Vec<MedModel> = MedModel::get_from_path("src/data/data.csv")?;

    let mut inputs: Vec<Vec<f32>> = vec![];
    let mut outputs: Vec<Vec<f32>> = vec![];
    models.iter_mut().for_each(|model| {
        let inp_out = model.to_param();
        outputs.push(vec![inp_out[0]]);
        inputs.push(inp_out[1..inp_out.len()].to_vec());
    });
    
    //let mut neural_net: Network = Network::new(vec![inputs[0].len(), inputs[0].len()*2, inputs[0].len()*3, inputs[0].len(), outputs[0].len()], triton_grow::network::activations::Activations::SIGMOID, 0.01);
    //neural_net = neural_net.train_to_loss(inputs.clone(), outputs.clone(), None, 0.1, 10000, triton_grow::network::modes::Mode::Avg, 0.01, 0.0001, inputs[0].len(), inputs[0].len()*3);
    //neural_net.train(&inputs, &outputs, 1000000, &triton_grow::network::modes::Mode::Avg);

    //neural_net.save("breast_model2.json");
    let mut neural_net: Network = Network::load("breast_model2.json");
    neural_net = neural_net.train_to_loss(inputs.clone(), outputs.clone(), None, 0.1, 100000, triton_grow::network::modes::Mode::Avg, 0.01, 0.0001, inputs[0].len(), inputs[0].len()*3);
    neural_net.save("breast_model3.json");
    for i in 0..inputs.len(){
        println!("Model predicted: {} \nActual: {}", neural_net.feed_forward(&inputs[i])[0], outputs[i][0]);
    }
    Ok(())
}
