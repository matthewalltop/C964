
export interface PlotlyResponse {
  traces: any[];
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
