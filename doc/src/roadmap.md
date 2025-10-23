# Roadmap to Olympus 0.1

The objectives of the first version of Olympus is to give a proof of concept of
a generic image processing library in Rust. To this aim, we plan to implement
the paper from
[here](https://pdfs.semanticscholar.org/1fc8/9dfd61e422b38d88565a00494a487f39de0d.pdf)
and [here](https://www.lrde.epita.fr/dload/papers/levillain.09.ismm.pdf).

However, as this library is a Research Driven library, some other
functionalities may become priorities.

- [X] Implementation of 2D image
- [X] Optimization of a 2D image
- [X] Implementation of the trait `Image`.
- [ ] Implementation of `Image` based on graph domain
    - [X] Node weighted graph
    - [ ] Edge weighted graph
- [ ] Implementation of simplicial complexes
- [ ] Implementation of `Image` based simplicial complexes
- [X] Implementation of morphological operations.
    - [X] Make it generic for any kind of `Image`
- [X] Implementation of morphological connected operators (based on Maxtree).
    - [ ] Make it generic for any kind of `Image`
- [X] Implementation of the morphological watershed from Meyer
  [here](https://people.cmm.minesparis.psl.eu/users/marcoteg/cv/publi_pdf/MM_refs/meyer/Meyer_watershed91.pdf).
    - [ ] Make it generic for any kind of `Image`
