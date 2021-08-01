pub mod plant_structures {
    pub mod roots {
        pub mod products {
            pub(in crate::plant_structures::roots) struct Cytokinin {}
        }
        use products::Cytokinin;
    }
    //use roots::products::Cytokinin;
}
