use std::error::Error;
use unda::{core::{data::{input::Input, matrix::Matrix, matrix3d::Matrix3D}, network::Network, layer::{layers::{LayerTypes, InputTypes}, methods::{activations::Activations, errors::ErrorTypes}}}, util::{mnist::MnistEntry, categorical::to_categorical}};

use rand::seq::SliceRandom;

use breast_cancer::models::med_model::MedModel;

fn main() -> Result<(), Box<dyn Error>>{
    let mut models: Vec<(MedModel, f32)> = MedModel::get_from_path("src/data/data.csv")?;
    let mut rng = rand::thread_rng();
    models.shuffle(&mut rng);

    let mut inputs: Vec<&dyn Input> = vec![];
    let mut outputs: Vec<Vec<f32>> = vec![];
    models.iter_mut().for_each(|model| {
        outputs.push(vec![model.1]);
        inputs.push(&model.0);
    });

    let mut network = Network::new(128);
    network.set_input(InputTypes::DENSE(inputs[0].to_param().len()));
    network.add_layer(LayerTypes::DENSE(16, Activations::RELU, 0.001));
    network.add_layer(LayerTypes::DENSE(1, Activations::SIGMOID, 0.001));

    network.compile();

    network.fit(&inputs, &outputs, 1, ErrorTypes::CategoricalCrossEntropy);

    for i in 0..10{
        println!("Model output: {:?}\nActual: {:?}", network.predict(&inputs[i].to_param()), outputs[i]);
    }

    Ok(())
}
