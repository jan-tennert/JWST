use bevy::{prelude::{Handle, Vec3}, scene::Scene};

use crate::body::BodyBundle;

const AU_TO_UNIT_SCALE: f32 = 10.0;

pub struct Body {
    pub model: Handle<Scene>,
    pub body: BodyBundle,
    pub radius: f32,
    pub model_scale: f32,
    pub name: String
}

impl Body {
    
    pub fn earth(earth_model: Handle<Scene>) -> Self {
        Body {
            model: earth_model,
            radius: 0.005,
            body: BodyBundle::new(
                5.97219,
                Vec3::new(
                    4.487758087146768E-01, 8.751235324844499E-01, 1.618817013329493E-04
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                    -1.552868871220300E-02, 7.906229533085379E-03, 3.064648367334892E-07
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.00001,
            name: "Earth".to_string()
        }
    }
    
    pub fn saturn(saturn_model: Handle<Scene>) -> Self {
        Body {
            model: saturn_model,
            radius: 0.005,
            body: BodyBundle::new(
                568.3,
                Vec3::new(
                    8.032503665636328E+00, -5.674419409731062E+00, -2.211472254846864E-01
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                    2.906095271828988E-03, 4.545286691593917E-03, -1.944528757086951E-04
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.00001,
            name: "Saturn".to_string()
        }
    }
    
    pub fn jupiter(jupiter_model: Handle<Scene>) -> Self {
        Body {
            model: jupiter_model,
            radius: 0.005,
            body: BodyBundle::new(
                1898.187,
                Vec3::new(
                  4.883310383356100E+00, 7.577598574024473E-01, -1.123963322175233E-01
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                   -1.243645389952230E-03, 7.811788737744427E-03, -4.555620902846121E-06
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.00001,
            name: "Jupiter".to_string()
        }
    }
    
    pub fn mars(mars_model: Handle<Scene>) -> Self {
        Body {
            model: mars_model,
            radius: 0.005,
            body: BodyBundle::new(
                0.64171,
                Vec3::new(
                  5.371347489929870E-01, 1.415777733841128E+00, 1.647268731293564E-02
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                   -1.252424659948937E-02, 6.220232033014156E-03, 4.378447959849454E-04
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.00001,
            name: "Mars".to_string()
        }
    }
    
    pub fn uranus(uranus_model: Handle<Scene>) -> Self {
        Body {
            model: uranus_model,
            radius: 0.005,
            body: BodyBundle::new(
                86.813,
                Vec3::new(
                  1.346817163779143E+01, 1.433467548071632E+01, -1.212433131756314E-01
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                   -2.895294668494246E-03, 2.509923332168401E-03, 4.682362547589839E-05
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.00001,
            name: "Uranus".to_string()
        }
    }
    
    pub fn mercury(mercury_model: Handle<Scene>) -> Self {
        Body {
            model: mercury_model,
            radius: 0.005,
            body: BodyBundle::new(
                0.3302,
                Vec3::new(
                  3.111405698823826E-03, -4.607974584403516E-01, -3.860159093638146E-02
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                   2.248220104774973E-02, 2.177869427789603E-03, -1.883369041847100E-03
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.00001,
            name: "Mercury".to_string()
        }
    }
    
    pub fn venus(venus_model: Handle<Scene>) -> Self {
        Body {
            model: venus_model,
            radius: 0.005,
            body: BodyBundle::new(
                4.867,
                Vec3::new(
                    -1.104602952742054E-01, -7.189512888891817E-01, -3.818331146763080E-03
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                    1.989263354357960E-02, -2.910502841093310E-03, -1.187594747165436E-03
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.00001,
            name: "Venus".to_string()
        }
    }
    
    pub fn pluto(pluto_model: Handle<Scene>) -> Self {
        Body {
            model: pluto_model,
            radius: 0.005,
            body: BodyBundle::new(
                0.0130900,
                Vec3::new(
                    -1.104602952742054E-01, -7.189512888891817E-01, -3.818331146763080E-03
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                    1.989263354357960E-02, -2.910502841093310E-03, -1.187594747165436E-03
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.00001,
            name: "Pluto".to_string()
        }
    }
    
    pub fn moon(moon_model: Handle<Scene>) -> Self {
        Body {
            model: moon_model,
            radius: 0.002,
            body: BodyBundle::new(
                0.0734767,
                Vec3::new(
                4.482115265952957E-01, 8.727621196450731E-01, 3.888179917645140E-05
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                -1.491883668334010E-02, 7.773993419863166E-03, -4.679176055656679E-05
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.000003,
            name: "Moon".to_string()
        }
    }
    
    pub fn jwst(jwst_model: Handle<Scene>) -> Self {
        Body {
            model: jwst_model,
            radius: 0.002,
            body: BodyBundle::new(
                6200.0e-24,
                Vec3::new(
                4.488948840878112E-01, 8.856860339483225E-01, -7.512566561474845E-04
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                -1.564139006806661E-02, 7.940335006606503E-03, -9.026475694712961E-05
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.000003,
            name: "JWST".to_string()
        }
    }
    
    pub fn hubble(hubble_model: Handle<Scene>) -> Self {
        Body {
            model: hubble_model,
            radius: 0.002,
            body: BodyBundle::new(
                11600.0 / f32::powf(10.0, 24.0),
                Vec3::new(
                4.487378270154649E-01, 8.751063507720495E-01, 1.817069672476064E-04
                ) * AU_TO_UNIT_SCALE,
                Vec3::new(
                -1.303624904379408E-02, 5.738929759261992E-03, 2.893730048309336E-03
                ) * AU_TO_UNIT_SCALE
            ),
            model_scale: 0.000003,
            name: "Hubble".to_string()
        }
    }
    
}