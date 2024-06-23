export interface SupportersData {
  _id: number,
  id: string,
  screen_id: string,
  name: string,
  image: string,
  profile: string,
  level: number,
  last_movie_id: string | null,
  is_live: boolean,
  supported: number,
  supporter_count: number,
  supporting_count: number,
  point: number,
  total_point: number,
  created: number,
}

export interface SupportersList {
  0: string;
  1: SupportersData[];
}