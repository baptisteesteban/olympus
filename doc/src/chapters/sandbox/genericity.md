# Genericity

This page describes the experiments located in the `/sandbox/genericity` folder.
These are attempts to reproduce some results described
[here](https://d1wqtxts1xzle7.cloudfront.net/44004988/Milena_Write_Generic_Morphological_Algor20160322-30607-11xdv73-libre.pdf?1458683072=&response-content-disposition=inline%3B+filename%3DMilena_Write_Generic_Morphological_Algor.pdf&Expires=1761334031&Signature=UGZ9lRfQxDuFqa65IKnFUsDGX3-62DNkHyyOwmFzimnF-hV2f9Caatwdl9wBXisHnbKWow9kYU8PaDhvbHsfG-ULtNLr7hNbAJ5elaUxkIJgwYuhDe6GT0OWNttsKpvPGv5nXWG-0g1wOJqR97rOXQbKxidSoCqcnIZage8B8oWU1XSw7Bl5BdiHPmNihYrZyEm~Nf0yaip1V3Y~46wFsrXv0a3rV6LH2DBR2sF7hJ6p4Y05pWPvb-IFz1AYGxNCAErtaOIS45Q02H67kV-YXZzenFVuGpBDfr9kwUwJp~LgZJaxeXlvOcg6V7~v0eiQCoxahnugf8N8sZZy5wpeqg__&Key-Pair-Id=APKAJLOHF5GGSLRBV4ZA).
More details may be found in [this
thesis](https://theses.hal.science/pastel-00673121v1).

<div style="text-align: center;">

<table style="border-collapse: collapse; margin: 0 auto;">
<tr>
    <th style="text-align: center; padding: 10px; font=bold;">Input Image</th>
    <th style="text-align: center; padding: 10px; font=bold;">Gradient</th>
    <th style="text-align: center; padding: 10px; font=bold;">Watershed</th>
</tr>
<tr>
    <td style="text-align: center; padding: 5px;"><img src="../../imgs/lena_gray.png" width="200"/></td>
    <td style="text-align: center; padding: 5px;"><img src="../../generated/lena_gradient.png" width="200"/></td>
    <td style="text-align: center; padding: 5px;"><img src="../../generated/lena_watershed.png" width="200"/></td>
</tr>
<tr>
    <td style="text-align: center; padding: 5px;"><img src="../../generated/node_graph.svg" width="200"/></td>
    <td style="text-align: center; padding: 5px;"><img src="../../generated/node_graph_grad.svg" width="200"/></td>
    <td style="text-align: center; padding: 5px;"><img src="../../generated/node_graph_watershed.svg" width="200"/></td>
</tr>
</table>

</div>