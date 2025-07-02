// rbei.EbpfIngress.c - Handles traffic TO BOTTLE container
// Rewrites source sentry->gateway for all inbound traffic

#include <linux/bpf.h>
#include <linux/if_ether.h>
#include <linux/ip.h>
#include <linux/in.h>
#include <linux/pkt_cls.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_endian.h>

// Config will be prepended by makefile
// Expected defines:
// #define RBE_GATEWAY_IP 0x...
// #define RBE_SENTRY_IP 0x...  
// #define RBE_GATEWAY_MAC {0x..., 0x..., ...}

#define ETH_ALEN 6
#define ARP_REQUEST 1
#define ARP_REPLY 2

struct arphdr_eth {
    __be16 ar_hrd;
    __be16 ar_pro;
    __u8   ar_hln;
    __u8   ar_pln;
    __be16 ar_op;
    __u8   ar_sha[ETH_ALEN];
    __be32 ar_sip;
    __u8   ar_tha[ETH_ALEN];
    __be32 ar_tip;
} __attribute__((packed));

SEC("tc")
int bottle_ingress(struct __sk_buff *skb)
{
    void *data = (void *)(long)skb->data;
    void *data_end = (void *)(long)skb->data_end;
    
    // Parse Ethernet header
    struct ethhdr *eth = data;
    if ((void *)(eth + 1) > data_end)
        return TC_ACT_SHOT;
    
    // Drop broadcast/multicast
    if (eth->h_dest[0] & 0x01)
        return TC_ACT_SHOT;
    
    // Handle ARP
    if (bpf_ntohs(eth->h_proto) == ETH_P_ARP) {
        struct arphdr_eth *arp = (void *)(eth + 1);
        if ((void *)(arp + 1) > data_end)
            return TC_ACT_SHOT;
            
        // Only handle ARP from sentry
        if (arp->ar_sip != RBE_SENTRY_IP)
            return TC_ACT_SHOT;
            
        // Rewrite to gateway
        __u8 gateway_mac[] = RBE_GATEWAY_MAC;
        arp->ar_sip = RBE_GATEWAY_IP;
        __builtin_memcpy(eth->h_source, gateway_mac, ETH_ALEN);
        
        if (bpf_ntohs(arp->ar_op) == ARP_REPLY) {
            __builtin_memcpy(arp->ar_sha, gateway_mac, ETH_ALEN);
        }
        
        // Direct packet modifications are already done above
        // No need for bpf_skb_store_bytes
        
        return TC_ACT_OK;
    }
    
    // Handle IPv4
    if (bpf_ntohs(eth->h_proto) == ETH_P_IP) {
        struct iphdr *ip = (void *)(eth + 1);
        if ((void *)(ip + 1) > data_end)
            return TC_ACT_SHOT;
            
        // Drop fragments
        if (bpf_ntohs(ip->frag_off) & 0x3FFF)
            return TC_ACT_SHOT;
            
        // Check source - must be from sentry
        if (ip->saddr != bpf_htonl(RBE_SENTRY_IP))
            return TC_ACT_SHOT;
            
        // Rewrite to gateway
        __u8 gateway_mac[] = RBE_GATEWAY_MAC;
        __builtin_memcpy(eth->h_source, gateway_mac, ETH_ALEN);
        ip->saddr = bpf_htonl(RBE_GATEWAY_IP);
        
        // Update checksums
        bpf_l3_csum_replace(skb, sizeof(*eth) + offsetof(struct iphdr, check),
                           bpf_htonl(RBE_SENTRY_IP), bpf_htonl(RBE_GATEWAY_IP), 4);
        
        // Direct packet modifications are already done above
        // No need for bpf_skb_store_bytes
        
        return TC_ACT_OK;
    }
    
    // Drop all other protocols
    return TC_ACT_SHOT;
}

char _license[] SEC("license") = "GPL";

