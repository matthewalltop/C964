
export interface PlotlyResponse {
  traces: any[];
  layout: any;
}

export interface TableData {
  columns: ColumnData[];
}

export class MLResponse {
  constructor(
  public cf_matrix: string,
  public accuracy: number,
  public precision: number,
  public recall: number,
  public threshold: number | null,
  public iterations: number | null,
  public tikz: string | null) {
    this.accuracy = Math.floor(+this.accuracy.toFixed(2) * 100);
    this.precision = Math.floor(+this.precision.toFixed(2) * 100);
    this.recall = Math.floor(+this.recall.toFixed(2) * 100);
    this.threshold = this.threshold ? Math.floor(+this.threshold.toFixed(2) * 100) : null;
  }
}

export interface ConfusionMatrix {
  headerRow: string[];
  rows: string[][];
}


interface ColumnData {
  name: string;
  dataType: string | object;
  bit_settings: string;
  values: any[];
}
