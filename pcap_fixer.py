# python pcap_fixer.py captura.pcap
# Generará 'captura_fixed.pcap'

import struct
import sys
import os

def fix_pcap_header(input_file, output_file):
    """
    Lee un archivo PCAP generado por secureot-agent y cambia el 
    LinkType de RAW (101) a ETHERNET (1) para que Wireshark lo lea bien.
    """
    try:
        with open(input_file, 'rb') as f_in:
            # Leer el Global Header (24 bytes)
            global_header = bytearray(f_in.read(24))
            
            if len(global_header) < 24:
                print(f"❌ Error: El archivo {input_file} es muy corto.")
                return

            # El LinkType está en los últimos 4 bytes del header (offset 20)
            # secureot-agent escribe 0x65 (101) que es LINKTYPE_RAW
            linktype = struct.unpack('<I', global_header[20:24])[0]
            
            print(f"ℹ️  Archivo: {input_file}")
            print(f"ℹ️  LinkType original detectado: {linktype}")

            if linktype == 101:
                print("⚠️  Detectado LINKTYPE_RAW (101). Cambiando a ETHERNET (1)...")
                # Cambiar a 1 (Ethernet)
                global_header[20:24] = struct.pack('<I', 1)
            elif linktype == 1:
                print("✅  El archivo ya es ETHERNET (1). No se necesitan cambios mayores.")
            else:
                print(f"⚠️  LinkType desconocido ({linktype}). Se forzará a Ethernet de todas formas.")
                global_header[20:24] = struct.pack('<I', 1)

            # Escribir el nuevo archivo
            with open(output_file, 'wb') as f_out:
                f_out.write(global_header)
                # Copiar el resto del archivo (paquetes) tal cual
                while True:
                    chunk = f_in.read(65536)
                    if not chunk:
                        break
                    f_out.write(chunk)
            
            print(f"🎉 ¡Éxito! Archivo corregido guardado como: {output_file}")
            print("   -> Ahora puedes abrirlo en Wireshark sin errores.")

    except FileNotFoundError:
        print(f"❌ Error: No se encuentra el archivo {input_file}")
    except Exception as e:
        print(f"❌ Error inesperado: {e}")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Uso: python pcap_fixer.py <archivo_original.pcap>")
    else:
        input_path = sys.argv[1]
        # Crear nombre de salida (ej: archivo_fixed.pcap)
        base, ext = os.path.splitext(input_path)
        output_path = f"{base}_fixed{ext}"
        
        fix_pcap_header(input_path, output_path)
