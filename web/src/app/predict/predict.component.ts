import { CommonModule } from '@angular/common';
import { Component, CUSTOM_ELEMENTS_SCHEMA, inject } from '@angular/core';
import { PredictService } from '@services/predict.service';
import { PredictNavBarComponent } from "./predict-nav-bar/predict-nav-bar.component";
import { PredictRequest } from '@models/requests';
import { PlotlyModule } from 'angular-plotly.js';

@Component({
  selector: 'app-predict',
  standalone: true,
  imports: [CommonModule, PredictNavBarComponent, PlotlyModule],
  schemas: [CUSTOM_ELEMENTS_SCHEMA],
  providers: [PredictService],
  templateUrl: './predict.component.html',
  styleUrl: './predict.component.scss'
})
export class PredictComponent {
  public predictSvc = inject(PredictService);

  public tikz: string = `
\\documentclass[margin=10pt]{standalone}
\\usepackage{tikz,forest}
\\usetikzlibrary{arrows.meta}
\\forestset{
default preamble={
before typesetting nodes={
  !r.replace by={[, coordinate, append]}
},
where n children=0{
  tier=word,
}{
  %diamond, aspect=2,
},
where level=0{}{
  if n=1{
    edge label={node[pos=.2, above] {Y}},
  }{
    edge label={node[pos=.2, above] {N}},
  }
},
for tree={
  edge+={thick, -Latex},
  s sep'+=2cm,
  draw,
  thick,
  edge path'={ (!u) -| (.parent)},
  align=center,
}
}
}\\begin{document}\\begin{forest}[Label: "Negative"]
\\node [anchor=north west] at (current bounding box.north east) {%
                \\begin{tabular}{c c c}
                  \\multicolumn{3}{@{}l@{}}{Legend:}\\
                  Imp.&:&Impurity decrease\\\end{tabular}};
	\\end{forest}
\\end{document}`


  submit(event: PredictRequest) {
    console.log(event);
  }
}
