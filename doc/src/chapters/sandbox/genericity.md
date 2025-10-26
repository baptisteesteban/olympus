# Genericity

This page describes the experiments located in the `/sandbox/genericity` folder.
These are attempts to reproduce some results described
[here](https://d1wqtxts1xzle7.cloudfront.net/44004988/Milena_Write_Generic_Morphological_Algor20160322-30607-11xdv73-libre.pdf?1458683072=&response-content-disposition=inline%3B+filename%3DMilena_Write_Generic_Morphological_Algor.pdf&Expires=1761334031&Signature=UGZ9lRfQxDuFqa65IKnFUsDGX3-62DNkHyyOwmFzimnF-hV2f9Caatwdl9wBXisHnbKWow9kYU8PaDhvbHsfG-ULtNLr7hNbAJ5elaUxkIJgwYuhDe6GT0OWNttsKpvPGv5nXWG-0g1wOJqR97rOXQbKxidSoCqcnIZage8B8oWU1XSw7Bl5BdiHPmNihYrZyEm~Nf0yaip1V3Y~46wFsrXv0a3rV6LH2DBR2sF7hJ6p4Y05pWPvb-IFz1AYGxNCAErtaOIS45Q02H67kV-YXZzenFVuGpBDfr9kwUwJp~LgZJaxeXlvOcg6V7~v0eiQCoxahnugf8N8sZZy5wpeqg__&Key-Pair-Id=APKAJLOHF5GGSLRBV4ZA).
More details may be found in [this
thesis](https://theses.hal.science/pastel-00673121v1).


The images below are the result of our process. The first image is the input
image, the second one is the gradient on which the watershed algorithm is
applied and whose result is the third image.

<div class="img-row">
  <img src="../../imgs/lena_gray.png" alt="Input" />
  <img src="../../generated/lena_gradient.png" alt="Gradient" />
  <img src="../../generated/lena_watershed.png" alt="Watershed" />
</div>
<br>
<div class="img-row">
  <img src="../../generated/node_graph.svg" alt="Input" />
  <img src="../../generated/node_graph_grad.svg" alt="Gradient" />
  <img src="../../generated/node_graph_watershed.svg" alt="Watershed" />
</div>

<script type="importmap">
  {
      "imports": {
          "three": "https://unpkg.com/three@0.153.0/build/three.module.js",
          "three/examples/jsm/controls/OrbitControls.js": "https://unpkg.com/three@0.153.0/examples/jsm/controls/OrbitControls.js"
      }
  }
</script>

<div data-src="../../imgs/spot.off" class="off-viewer"></div>
<script type="module" src="../../js/init-viewers.js"></script>