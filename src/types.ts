export interface ServiceInfo {
  name: string;
  display_name: string;
  status: 'running' | 'stopped' | 'paused' | 'unknown';
  is_nssm: boolean;
  image_path?: string;
}
