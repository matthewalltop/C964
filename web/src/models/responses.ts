
export interface PlotlyGraph {
  // TODO: Not sure I want to create a full model for this.
  data: any[];
  layout: any;
}

export interface TableData {
  columns: ColumnData[];
}

interface ColumnData {
  name: string;
  dataType: string | object;
  bit_settings: string;
  values: any[];
}
