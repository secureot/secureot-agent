from scapy.all import rdpcap, Ether, IP, TCP, UDP
import sys
from collections import Counter

# Diccionario de puertos comunes en OT/ICS
OT_PORTS = {
    502: "Modbus TCP",
    102: "Siemens S7Comm",
    44818: "EtherNet/IP",
    2222: "EtherNet/IP (UDP)",
    47808: "BACnet",
    20000: "DNP3",
    2404: "IEC 104",
    1911: "Foxboro"
}

def analyze_pcap(pcap_file):
    print(f"🔍 Analizando: {pcap_file} ...")
    
    try:
        # Scapy es inteligente, intentará leerlo aunque el header sea RAW
        packets = rdpcap(pcap_file)
    except Exception as e:
        print(f"❌ Error leyendo PCAP (Prueba usar primero el pcap_fixer.py): {e}")
        return

    stats = {
        "total": 0,
        "ot_protocols": Counter(),
        "src_ips": Counter(),
        "dst_ips": Counter()
    }

    for pkt in packets:
        stats["total"] += 1
        
        # Intentar extraer capas IP
        if IP in pkt:
            stats["src_ips"][pkt[IP].src] += 1
            stats["dst_ips"][pkt[IP].dst] += 1
            
            # Detectar Protocolos OT basado en puertos
            dport = 0
            sport = 0
            
            if TCP in pkt:
                dport = pkt[TCP].dport
                sport = pkt[TCP].sport
            elif UDP in pkt:
                dport = pkt[UDP].dport
                sport = pkt[UDP].sport
                
            # Verificar si origen o destino es un puerto OT conocido
            if dport in OT_PORTS:
                stats["ot_protocols"][OT_PORTS[dport]] += 1
            elif sport in OT_PORTS:
                stats["ot_protocols"][OT_PORTS[sport]] += 1

    # --- Reporte ---
    print("\n" + "="*40)
    print(f"📊 REPORTE DE TRÁFICO OT: {pcap_file}")
    print("="*40)
    print(f"📦 Total Paquetes: {stats['total']}")
    
    print("\n🏭 Protocolos Industriales Detectados:")
    if stats["ot_protocols"]:
        for proto, count in stats["ot_protocols"].most_common():
            print(f"   - {proto}: {count} paquetes")
    else:
        print("   (No se detectaron protocolos OT estándar conocidos)")

    print("\n📡 Top 3 IPs Origen (Talkers):")
    for ip, count in stats["src_ips"].most_common(3):
        print(f"   - {ip}: {count}")

    print("\n🎯 Top 3 IPs Destino:")
    for ip, count in stats["dst_ips"].most_common(3):
        print(f"   - {ip}: {count}")
    print("="*40 + "\n")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Uso: python analyze_ot_traffic.py <archivo.pcap>")
    else:
        analyze_pcap(sys.argv[1])
